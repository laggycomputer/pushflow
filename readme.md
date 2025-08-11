# Railway Template – PushFlow: Hassle-Free WebPush Notifications

Web Push notifications (i.e. the ones you grant permission to and receive through the browser) require a lot of setup to
send.
There are a confusing set of keys and other strings you have to retain and send or not send to various parties.
It becomes even more annoying to deal with multiple sets of subscribers which should be notified en bloc in different
circumstances.

While Web Push documentation has improved over the last few years, it still does not plainly state the single way
forward for a developer.

We introduce a plug-and-play system that lets you quickly generate code to streamline the whole process:

1. On the user's browser, generating the information necessary for a subscription.
2. In the service database, storing which notifications that subscription should receive.
3. A web dashboard to manage this and create API keys.
4. An API surface to use these keys to add subscriptions or send notifications.

All of this, while having control of your own data.

## About Hosting // Implementation Details

We utilize the following:

* PostgreSQL for persistent data, *e.g.* API keys, dashboard users
* Redis for session storage behind the web dashboard
* A Rust backend responsible for OAuth-authenticated access from the web dashboard and key-based access from API keys,
  built with `actix-web`.
    * Google OAuth is used to provide identity.
* A Next.js frontend for the web dashboard, interfacing with the backend via SSR.

## Use Cases

* You want to manage push notification subscriptions to many categories specifically; *e.g.* users may subscribe to
  events pertaining to a specific entity in your application.
* You want to send push notifications or add new subscribers by REST, without fine-grained concern for storage or a data
  model.
* You need to monitor and manage subscribers in a graphical interface, and do not want to write your own.

## Dependencies + Implementation Details

The `.env.example` files are meant to ease local development/deployment and are not required; the template will set most
environment variables out of the box.

What you do need to do:
* You must set up Google OAuth: https://developers.google.com/identity/protocols/oauth2/web-server#enable-apis
* **You should set `$JWT_SECRET` to a strong passphrase.** This passphrase protects all session creation on the
  dashboard. Failure to do so may result in CSRF attacks by guessing of certain secrets in the OAuth flow.

## Why Deploy on Railway

* No limit on the number of notifications or subscribers; you pay the hosting cost directly, so no notification service
  can paywall you.
* You control your data, completely. You can monitor the situation, down to the database rows, at any time.
* You can fully understand the layout of the service with Railway's architecture visualization.
* Since the code is public, you can see exactly how data is handled (look, no telemetry!)
