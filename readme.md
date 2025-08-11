# pushflow

do web notifs good

## setup

The `.env.example` files are meant to ease local development/deployment.
In the Railway context, setting environment variables would be preferred.
You can use these files as an exhaustive list of required environment variables.

You will need to do some setup:
* You must set up Google OAuth: https://developers.google.com/identity/protocols/oauth2/web-server#enable-apis
* **You should set `$JWT_SECRET`.** This passphrase protects all authorization in the app. 



---

# Railway Template – PushFlow: Hassle-Free WebPush Notifications

something about how Push Notification usually require a bunch of setup or are annoying to deal with

problem that webpush documentation, while having improved over the last few years, is still not super common.

Therefore, we want easy deployment system that lets you quickly generate code to streamline the whole process: generating a subscription -> tracking web push subscriptions across all the different apps/services you create -> easy API-based delivery. All of this, while having control of your own data.

## About Hosting // Implementation Details

the tech stack or some stuff

## Use Cases

## Dependencies + Implementation Details
probably skip this since this seems to be for people porting ther stuff *to* railway

## Why Deploy on Railway

- you are in control of your data
- No limit on the number of notifications you can send
- something
