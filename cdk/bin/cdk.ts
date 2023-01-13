#!/usr/bin/env node

import * as cdk from 'aws-cdk-lib';
import {StaticSiteCdkStack} from '../lib/static-site-cdk-stack';

const app = new cdk.App();
const environment = {account: '519160639284', region: 'us-west-2'}

const privacyPolicyDomainName = 'ezmempass-privacy-policy.ihsan.io'
const privacyPolicySourceFolder = '../docs/privacy_policy/'
new StaticSiteCdkStack(
    app,
    'prod-EzMemPassPrivacyPolicyStack',
    privacyPolicyDomainName,
    privacyPolicySourceFolder,
    {env: environment}
);

const supportSiteDomainName = 'ezmempass.ihsan.io'
const supportSiteSourceFolder = '../docs/support_site/'
new StaticSiteCdkStack(
    app,
    'prod-EzMemPassSupportSiteStack2',
    supportSiteDomainName,
    supportSiteSourceFolder,
    {env: environment}
);