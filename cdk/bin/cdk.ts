#!/usr/bin/env node
import * as cdk from '@aws-cdk/core';
import { CdkStack } from '../lib/cdk-stack';

const app = new cdk.App();
const environment = { account: '519160639284', region: 'us-west-2' }
const domainName = 'ezmempass-privacy-policy.ihsan.io'

new CdkStack(app, 'prod-EzMemPassPrivacyPolicyStack', domainName, { env: environment });
