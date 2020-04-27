import * as cdk from '@aws-cdk/core';

import * as certificatemanager from '@aws-cdk/aws-certificatemanager';
import * as cloudfront from '@aws-cdk/aws-cloudfront';
import * as route53 from '@aws-cdk/aws-route53';
import * as s3 from '@aws-cdk/aws-s3';
import * as s3deployment from '@aws-cdk/aws-s3-deployment';
import * as targets from '@aws-cdk/aws-route53-targets';

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
    const distribution = new cloudfront.CloudFrontWebDistribution(this, 'SiteDistribution', {
      viewerCertificate: cloudfront.ViewerCertificate.fromAcmCertificate(certificate, {
        aliases: [domainName],
        securityPolicy: cloudfront.SecurityPolicyProtocol.TLS_V1_2_2018,
        sslMethod: cloudfront.SSLMethod.SNI,
      }),
      originConfigs: [
        {
          s3OriginSource: {
            s3BucketSource: siteBucket
          },
          behaviors: [{ isDefaultBehavior: true, compress: true }],
        }
      ],
      priceClass: cloudfront.PriceClass.PRICE_CLASS_100,
      viewerProtocolPolicy: cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS
    });
    // ------------------------------------------------------------------------

    // ------------------------------------------------------------------------
    //  Route 53 alias record pointing at the CloudFront distribution.
    // ------------------------------------------------------------------------
    new route53.ARecord(this, 'SiteAliasRecord', {
      zone: hostedZone,
      recordName: domainName + ".",
      target: route53.RecordTarget.fromAlias(new targets.CloudFrontTarget(distribution))
    });
    // ------------------------------------------------------------------------

    // ------------------------------------------------------------------------
    //  Actually deploy the site
    // ------------------------------------------------------------------------
    new s3deployment.BucketDeployment(this, 'DeployWithInvalidation', {
      sources: [s3deployment.Source.asset(sourceFolder)],
      destinationBucket: siteBucket,
      distribution,
      distributionPaths: ['/*'],
    });
    // ------------------------------------------------------------------------
  }
}
