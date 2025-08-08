# pushflow

do web notifs good

## setup

The `.env.example` files are meant to ease local development/deployment.
In the Railway context, setting environment variables would be preferred.
You can use these files as an exhaustive list of required environment variables.

You will need to do some setup:
* You must set up Google OAuth: https://developers.google.com/identity/protocols/oauth2/web-server#enable-apis
* **You should set `$JWT_SECRET`.** This passphrase protects all authorization in the app. 