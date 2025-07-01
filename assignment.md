# Assignment

The purpose of the assignment is to create a simple application that can create a shipment using our Sandbox Environment through our REST API, and keeps tracking of the actions performed by the application.

Your application must cover the following aspects:

It must securely authenticate with the keys provided by your contact at Shipmondo.

It must be able to create a simple shipment. It is fine that the body of the shipment request is hardcoded.

It must have a local database that keeps track of:

- Your account balance.
- Package number and ID of the shipment.

It must retrieve the balance from the API, and keep track of the balance of the account as it is updated when creating shipments.

The focus of the assignment is exclusively on backend code, so it should be without any UI. It is not necessary to have any API endpoints, looking at the database is enough.

You can use whatever language and framework that you prefer.

You can find all the necessary endpoints needed for your application from our API reference [here](https://shipmondo.dev/api-reference#/)
