# Steps to run the project

## Set up the wrangler cli

> you can follow [this](https://developers.cloudflare.com/workers/get-started/guide) tutorial or use below commands.

```bash

# for npm users
npm install -g @cloudflare/wrangler

# for yarn users
yarn global add @cloudflare/wrangler

# check if it's installed correctly
wrangler --version

# login to your account
wrangler login
```

## Clone this repository

```bash
# clone this repo
git clone git@github.com:intelliconnect/cloudflare-serverless-rust.git

# change to the project directory
cd cloudflare-serverless-rust
```

## Get your account id

> get your account id from your dashbord or use below command.

```bash
wrangler whoami

👋  You are logged in with an API Token, associated with the email '<Your Email>'.

+----------------------------------+----------------------------------+
| Account Name                     | Account ID                       |
+----------------------------------+----------------------------------+
| Your Account                     | $yourAccountId                   |
+----------------------------------+----------------------------------+
```

Put this account id into wrangler.toml file.

```text
# wrangler.toml
name = "my-wrangler-app"
main = "build/worker/shim.mjs"
workers_dev = true
compatibility_date = "2023-01-10"
account_id = "enter your account id"
```

## Create a kv store

> You can use [this](https://developers.cloudflare.com/workers/tutorials/workers-kv-from-rust) tutorial for more details.

```bash
# create kv store with name KV_FROM_RUST
wrangler kv:namespace create "KV_FROM_RUST"

add the following to the configuration file:

kv_namespaces = [
          { binding = "KV_FROM_RUST", id= "**********" }
]
```

add the above config to wrangler.toml

## Get the Data api secret key for mongodb

Goto the mongodb dashboard. in the left navigation bar under data service click on the Data API link.
create a API key and ***store it somewhere secure*** since, it will appear only once and it has access to the database.

***In the lib.rs file change your cluster name, database name and the collection name.***

### Create a secret in wrangler to store the secret key

> We will create a secret in wrangler to store the Data API key.

```bash
# create a secret with name mongo_data_api_key
wrangler secret put mongo_data_api_key

# it will ask for secret key paste it and press enter

```
You also need to create a ```.dev.vars``` filename in your root directory and paste your secret key (It is the value which you will create to store secret value) and Secret key as well. By doing this you can use this secret in the workers' code with just a function call.  

```bash

# create a filename as ‘.dev.vars’ in your root directory 
# Paste your secret value and key here like below 
mongo_data_api_key="your api key here" 

```

store the name of the key in wrangler.toml file and comment it out. it's just for refrence purpose.

```toml
# [SECRETS] (this is just for refrence)
# mongo_data_api_key
```

## Running the project

```bash
# compiles your project to WebAssembly and will warn of any issues
wrangler build 

# run your Worker in an ideal development workflow (with a local server, file watcher & more)
wrangler dev

# access the endpoint at 127.0.0.1:8787
# test the endpoint
curl 127.0.0.1:8787
#should return: Hello from Workers!


# post json data to the mongodb with POST request to 127.0.0.1:8787/data and the json body to post
# get the data from mongodb with GET request to 127.0.0.1:8787/data
# get the filtered data from mongodb with GET request to 127.0.0.1:8787/filtered_data

# deploy your Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
wrangler publish

# after publishing your url will be like my-wrangler-app.{your subdomain}.workers.dev

```

When the workers_dev field in wrangler.toml is true, it is published to *.workers.dev domain.

to publish it to a registered domain refer [here](https://developers.cloudflare.com/workers/get-started/guide#optional-configure-for-deploying-to-a-registered-domain).

----

## To create a new project for cloudflare in rust

```bash
# you can replace my-wrangler-app with your app name
wrangler generate my-wrangler-app https://github.com/cloudflare/rustwasm-worker-template
```
