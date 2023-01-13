# EzMemPass

## Download

<a href="https://apps.apple.com/us/app/ezmempass/id1490538369"><img src="app-store-button.png" alt="Download on the App Store" width="200" height="60" /></a>
<a href="https://play.google.com/store/apps/details?id=com.asimihsan.ezmempass&gl=US"><img src="google-play-badge.png" alt="Get it on Google Play" width="200" height="60" /></a>


## Introduction

Using EzMemPass you can create strong, easy to remember passwords. Use these passwords for very important services like your email account, or your password manager.

The **memory aid** is made up of words, and the first three letters of each word make up your **password**.

For example, a password that you use could be:

> foobanmadavastoboxloc

and a corresponding memory aid that you memorize could be:

> food bank made available strong box lock

The password on its own is difficult to remember. However with a memory aid the password is easier to remember. The password is the first three letters of each word. Knowing the words does not make the password weaker because EzMemPass generates random three-letter prefixes from 1024 possible prefixes; see below for details.

## Strength of passwords

Use 7 or more words for regular use, 10 or more words for very important passwords such as for your **password manager** or **email account**.

For a password with 7 words, EzMemPass chooses 7 three-letter prefixes out of 1024 possible prefixes. Hence a 7-word password is chosen from `1024 ^ 7 = 1.18 * 10^21 = 2^70` possible combinations of prefixes. Someone attempting to guess a 7 word password must guess `2^70` times.

Adding capital letters, digits, or symbols is there just to satisfy the password rules of some services and websites.

## How to use passwords

Use a **password manager** like 1Password, LastPass, or BitWarden to store your passwords. Generate **completely random passwords for individual websites** using your password manager. Then use **one strong EzMemPass 10-word password for your password manager**.

I recommend also generating 7 word EzMemPass passwords for critical accounts like your email account and social media accounts just in case you forget or lose access to your password manager.

Practice typing in passwords at least twice a day for a week when you first create an EzMemPass password. This helps you memorize the password. Store both the password and the memory aid in your password manager.

## How EzMemPass creates passwords

- Find the 1024 most popular three-letter prefixes in the language, e.g. English.
- Choose N prefixes at random
- Try to come up with a plausible sequence of words that match each prefix.

The passwords are easier to remember than random characters because EzMemPass provides a memory aid that looks like a regular sentence. However, the password is more secure then choosing a real sentence from a language because EzMemPass chooses prefixes at random.

## Support and contact details

Please send comments, questions, and feedback to `ezmempass@gmail.com`.