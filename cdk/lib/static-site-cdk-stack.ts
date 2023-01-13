import * as cdk from 'aws-cdk-lib';
import {
    aws_certificatemanager as certificatemanager,
    aws_cloudfront as cloudfront,
    aws_cloudfront_origins as origins,
    aws_route53 as route53,
    aws_route53_targets as route53_targets,
    aws_s3 as s3,
    aws_s3_deployment as s3_deployment
} from 'aws-cdk-lib';

export class StaticSiteCdkStack extends cdk.Stack {
    constructor(scope: cdk.App, id: string, domainName: string, sourceFolder: string, props?: cdk.StackProps) {
        super(scope, id, props);

        // ------------------------------------------------------------------------
        //  Public S3 bucket containing the static site.
        // ------------------------------------------------------------------------
        const siteBucket = new s3.Bucket(this, 'SiteBucket', {
            bucketName: domainName,
            websiteIndexDocument: 'index.html',
            websiteErrorDocument: 'error.html',
            publicReadAccess: true,
            removalPolicy: cdk.RemovalPolicy.DESTROY,
        });
        // ------------------------------------------------------------------------

        // ------------------------------------------------------------------------
        //  Certificate.
        // ------------------------------------------------------------------------
        const hostedZone = route53.HostedZone.fromLookup(this, 'HostedZone', {
            domainName: 'ihsan.io',
            privateZone: false
        });
        const certificate = new certificatemanager.DnsValidatedCertificate(this, "Certificate", {
            domainName: domainName,
            region: "us-east-1",
            hostedZone: hostedZone
        });
        // ------------------------------------------------------------------------

        // ------------------------------------------------------------------------
        //  CloudFront distribution in front of the S3 bucket.
        // ------------------------------------------------------------------------
        const distribution = new cloudfront.Distribution(this, 'SiteDistribution', {
            defaultBehavior: {
                origin: new origins.S3Origin(siteBucket),
                viewerProtocolPolicy: cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS
            },
            certificate: certificate,
            domainNames: [domainName],
            priceClass: cloudfront.PriceClass.PRICE_CLASS_100,
        });

        // Upgrade! https://docs.aws.amazon.com/cdk/api/v1/docs/aws-cloudfront-readme.html#migrating-from-the-original-cloudfrontwebdistribution-to-the-newer-distribution-construct
        const cfnDistribution = distribution.node.defaultChild as cloudfront.CfnDistribution;
        cfnDistribution.overrideLogicalId('SiteDistributionCFDistribution209CF7F5');
        // ------------------------------------------------------------------------

        // ------------------------------------------------------------------------
        //  Route 53 alias record pointing at the CloudFront distribution.
        // ------------------------------------------------------------------------
        new route53.ARecord(this, 'SiteAliasRecord', {
            zone: hostedZone,
            recordName: domainName + ".",
            target: route53.RecordTarget.fromAlias(new route53_targets.CloudFrontTarget(distribution))
        });
        // ------------------------------------------------------------------------

        // ------------------------------------------------------------------------
        //  Actually deploy the site
        // ------------------------------------------------------------------------
        new s3_deployment.BucketDeployment(this, 'DeployWithInvalidation', {
            sources: [s3_deployment.Source.asset(sourceFolder)],
            destinationBucket: siteBucket,
            distribution,
            distributionPaths: ['/*'],
            memoryLimit: 1024,
        });
        // ------------------------------------------------------------------------
    }
}
