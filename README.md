<!-- 

Github:
https://github.com/degu0055/25W_LabAssignment?tab=readme-ov-file 

-->

# 25W_LabAssignment

## Architecture Diagram
<!-- Draw the updated architecture diagram using Draw.io and include it in the README. -->
![Diagram](https://github.com/degu0055/25W_LabAssignment/raw/main/images/diagram2.png)

## Application and Architecture Explanation
<!-- Briefly explain the application functionality and how the architecture works. -->

### Customer 
Customers can view a list of products on the homepage. The homepage displays product images, prices, and additional details. There is an “Add to Cart” button that allows customers to collect their orders. They can add items, edit their selections, or remove products before finalizing the order.

- **Store-front**: A user interface where clients can explore and purchase items 
- **Order-service**: Handles customer orders and communicates with Azure Service Bus for message queuing  
- **Product-service**: Manages product listings, details, and inventory 
- **Order queue**: A message broker helps services communicate with each other. It queues orders for processing  

### Employee/Admin
For this web app to function properly, an admin must first create a product list, allowing customers to view available products and place orders. Once an order is received, the admin can access the order details for processing, which will allow them to clear the logs later.

- **Store-admin**: A user interface where employee or admin can add and edit a product
- **Makeline-service**: Retrieve the order details from the service bus and then proceed to process the order
- **Order-database**: A data storage for the product  
- **AI-service**  
  - **Large Language Model**: A chatGPT 4 that can offer recommendations specifically regarding product information.
  - **Image Generator Model**: To create an image derived from the tag and product description. 


## Deployment Instructions
<!-- Step-by-step instructions to deploy the application in a Kubernetes cluster. -->

> **Note:** In order to use Azure services, you will need the following credentials:
> 
> - **Endpoint**: This is the URL that allows you to connect to the Azure service.
> - **Secret**: This is a secure key used for authentication to ensure that only authorized users can access the Azure service.
> 
> Both the endpoint and secret are typically provided when setting up Azure services like Azure service bus



### Prerequisite:
1. [Installed kubectl](https://github.com/ramymohamed10/Lab6_25W_CST8915/blob/main/README.md)
2. [Azure Kubernetes Cluster](https://github.com/ramymohamed10/Lab6_25W_CST8915/blob/main/README.md)

### Quick Setup (if you want to skip #2 prerequisite)
This can be done using terminal

Resource Group:
```sh 
az group create \
--name BestBuyRg \
--location canadacentral
```
Kubernetes and masterpool:
```sh
  az aks create \
--resource-group BestBuyRg \
--name BestBuyCluster \
--location canadacentral \
--nodepool-name masterpool \
--node-vm-size Standard_D2as_v4 \
--node-count 1 \
--generate-ssh-keys \
--vm-set-type VirtualMachineScaleSets \
--load-balancer-sku standard \
--tier Free \
--network-plugin azure \
--no-wait
```

Woorkpool:
```sh
  az aks nodepool add \
--resource-group BestBuyRg \
--cluster-name BestBuyCluster \
--name workerspool \
--node-vm-size Standard_D2as_v4 \
--node-count 1 \
--mode User \
--no-wait
```

Login to acces kubernetes:
```sh 
az login
az account set --subscription 'subscribtion-id'
az aks get-credentials --resource-group AlgonquinPetStoreRG --name AlgonquinPetStoreCluster
```

### Task 1: Connect Order-service to Azure Service Bus

Create Azure Resources

<!-- ```sh
az group create --name <resource-group-name> --location <location>
az servicebus namespace create --name <namespace-name> --resource-group <resource-group-name>
az servicebus queue create --name orders --namespace-name <namespace-name> --resource-group <resource-group-name>
``` -->

```sh
az group create --name BestBuyRg --location canadacentral
az servicebus namespace create --name BestBuyNamespace --resource-group BestBuyRg
az servicebus queue create --name orders --namespace-name BestBuyNamespace --resource-group BestBuyRg
```

Set Up Authentication
- Using Managed Identity (Recommended)

<!-- ```sh
PRINCIPALID=$(az ad signed-in-user show --query objectId -o tsv)
SERVICEBUSBID=$(az servicebus namespace show --name <namespace-name> --resource-group <resource-group-name> --query id -o tsv)

az role assignment create --assignee john.doe@example.com --role "Azure Service Bus Data Sender" --scope /subscriptions/{subscription-id}/resourceGroups/{resource-group-name}/providers/Microsoft.ServiceBus/namespaces/{namespace-name}
``` -->

<!-- ```sh Old
PRINCIPALID=$(az ad signed-in-user show --query objectId -o tsv)
SERVICEBUSBID=$(az servicebus namespace show --name BestBuyNamespace --resource-group BestBuyRg --query id -o tsv)

az role assignment create --assignee degu0055@algonquinlive.com --role "Azure Service Bus Data Sender" --scope /subscriptions/cdb9bdf3-e7ee-43e9-8f6c-ba7327868df1/resourceGroups/BestBuyRg/providers/Microsoft.ServiceBus/namespaces/BestBuyNamespace
``` -->

```sh
SERVICEBUSBID=$(az servicebus namespace show --name BestBuyNamespace --resource-group BestBuyRg --query id -o tsv)

az role assignment create \
  --assignee degu0055@algonquinlive.com \
  --role "Azure Service Bus Data Sender" \
  --scope $SERVICEBUSBID

```

- Save Environment Variables

<!-- ```sh
HOSTNAME=$(az servicebus namespace show --name <namespace-name> --resource-group <resource-group-name> --query serviceBusEndpoint -o tsv | sed 's/https:\/\///;s/:443\///')

cat << EOF > .env
USE_WORKLOAD_IDENTITY_AUTH=true
AZURE_SERVICEBUS_FULLYQUALIFIEDNAMESPACE=$HOSTNAME
ORDER_QUEUE_NAME=orders
EOF

source .env # to push the variable on messagequeue.js

``` -->

```sh
HOSTNAME=$(az servicebus namespace show --name BestBuyNamespace --resource-group BestBuyRg --query serviceBusEndpoint -o tsv | sed 's/https:\/\///;s/:443\///')

### By typing this, make sure you're in right order-service folder or directory
cat << EOF > .env
USE_WORKLOAD_IDENTITY_AUTH=true
AZURE_SERVICEBUS_FULLYQUALIFIEDNAMESPACE=$HOSTNAME
ORDER_QUEUE_NAME=orders
EOF

source .env # to push the variable on messagequeue.js
```

Push to your docker hub, this will be later use on aps-all-in-one.yaml
```sh
docker buildx build --platform linux/amd64 -t degu0055/order-service-bestbuy:latest --push .
```

Create an authorization-rule
```sh
az servicebus queue authorization-rule create --name sender --namespace-name BestBuyNamespace --resource-group BestBuyRg --queue-name orders --rights Send
```

- Using Shared Access Policy (Alternative)

<!-- ```sh
az servicebus queue authorization-rule create --name sender --namespace-name <namespace-name> --resource-group <resource-group-name> --queue-name orders --rights Send

HOSTNAME=$(az servicebus namespace show --name <namespace-name> --resource-group <resource-group-name> --query serviceBusEndpoint -o tsv | sed 's/https:\/\///;s/:443\///')
PASSWORD=$(az servicebus queue authorization-rule keys list --namespace-name <namespace-name> --resource-group <resource-group-name> --queue-name orders --name sender --query primaryKey -o tsv)

cat << EOF > .env
ORDER_QUEUE_HOSTNAME=$HOSTNAME
ORDER_QUEUE_PORT=5671
ORDER_QUEUE_USERNAME=sender
ORDER_QUEUE_PASSWORD="$PASSWORD"
ORDER_QUEUE_TRANSPORT=tls
ORDER_QUEUE_RECONNECT_LIMIT=10
ORDER_QUEUE_NAME=orders
EOF

source .env
``` -->

```sh
az servicebus queue authorization-rule create --name sender --namespace-name BestBuyNamespace --resource-group BestBuyRg --queue-name orders --rights Send

HOSTNAME=$(az servicebus namespace show --name BestBuyNamespace --resource-group BestBuyRg --query serviceBusEndpoint -o tsv | sed 's/https:\/\///;s/:443\///')
PASSWORD=$(az servicebus queue authorization-rule keys list --namespace-name BestBuyNamespace --resource-group BestBuyRg --queue-name orders --name sender --query primaryKey -o tsv)

cat << EOF > .env
ORDER_QUEUE_HOSTNAME=$HOSTNAME
ORDER_QUEUE_PORT=5671
ORDER_QUEUE_USERNAME=sender
ORDER_QUEUE_PASSWORD="$PASSWORD"
ORDER_QUEUE_TRANSPORT=tls
ORDER_QUEUE_RECONNECT_LIMIT=10
ORDER_QUEUE_NAME=orders
EOF

source .env
```

Create an authorization-rule
```sh
az servicebus queue authorization-rule create --name sender --namespace-name <namespace-name> --resource-group <resource-group-name> --queue-name orders --rights Send
```

### Task 2: Connect Makeline service to Azure Service Bus

```sh
az servicebus namespace authorization-rule create --name listener --namespace-name <namespace-name> --resource-group $RGNAME --rights Listen

HOSTNAME=$(az servicebus namespace show --name <namespace-name> --resource-group $RGNAME --query serviceBusEndpoint -o tsv | sed 's/https:\/\///;s/:443\///')
PASSWORD=$(az servicebus namespace authorization-rule keys list --namespace-name <namespace-name> --resource-group $RGNAME --name listener --query primaryKey -o tsv)

export ORDER_QUEUE_URI=amqps://$HOSTNAME
export ORDER_QUEUE_USERNAME=listener
export ORDER_QUEUE_PASSWORD=$PASSWORD
export ORDER_QUEUE_NAME=orders
```


### Task 3: Set Up the AI Backing Services
To enable AI-generated product descriptions and image generation features, you will deploy the required Azure OpenAI Services for GPT-4 (text generation) and DALL-E 3 (image generation). This step is essential to configure the AI Service component in the Algonquin Pet Store application.

#### Create an Azure OpenAI Service Instance
**Navigate to Azure Portal:**  
- Go to the Azure Portal.  

**Create a Resource:**  
- Select **Create a Resource** from the Azure portal dashboard.  
- Search for **Azure OpenAI** in the marketplace.  

**Set Up the Azure OpenAI Resource:**  
- Choose the **East US** region for deployment to ensure capacity for GPT-4 and DALL-E 3 models.  
- Fill in the required details:  
  - **Resource group**: Use an existing one or create a new group.  
  - **Pricing tier**: Select **Standard**.  

**Deploy the Resource:**  
- Click **Review + Create** and then **Create** to deploy the Azure OpenAI service.  

### Task 4: Deploy the GPT-4 and DALL-E 3 Models
#### Access the Azure OpenAI Resource:
- Navigate to the **Azure OpenAI** resource you just created.  

#### Deploy GPT-4:
- Go to the **Model Deployments** section and click **Add Deployment**.  
- Choose **GPT-4** as the model and provide a deployment name (e.g., `gpt-4-deployment`).  
- Set the deployment configuration as required and deploy the model.  

#### Deploy DALL-E 3:
- Repeat the same process to deploy **DALL-E 3**.  
- Use a descriptive deployment name (e.g., `dalle-3-deployment`).  

#### Note Configuration Details:
Once deployed, note down the following details for each model:  
- **Deployment Name**  
- **Endpoint URL**  

### Task 5: Retrieve and Configure API Keys
#### 1. Get API Keys:
- Go to the **Keys and Endpoints** section of your Azure OpenAI resource.
- Copy the **API Key (API key 1)** and **Endpoint URL**.  

#### 2. Base64 Encode the API Key:
```sh
echo -n "<your-api-key>" | base64
```
Replace `<your-api-key>` with your actual API key.

### Task 6: Update AI Service, Order Servive, & Makeline Service  Deployment Configuration in the Deployment Files folder
#### 1. Modify Secrets YAML:
- Edit the `secrets-AI.yaml`, `secrets-orderService.yaml`, * `secrets-makeline.yaml` file.
- Replace `OPENAI_API_KEY` placeholder with the Base64-encoded value of the `<your-api-key>`.
- Replace `connectionString` placeholder with the Base64-encoded value of the `<your-api-key>`.

#### 2. Modify Deployment YAML:
- Edit the `aps-all-in-one.yaml` file.
- Remove RabbitMQ configuration
- Replace the placeholders with the configurations you retrieved:
  ```yaml

  # Configuration for Order-service
  # Make sure to remove the rabbitMQ since this use azure service bus
  - name: AZURE_SERVICE_BUS_CONNECTION_STRING
  valueFrom:
    secretKeyRef:
      name: azure-servicebus-secret
      key: connectionString
  - name: AZURE_SERVICE_BUS_QUEUE_NAME
    value: "<QUEUE_NAME>"
  - name: AZURE_SERVICE_BUS_TOPIC_NAME
    value: "<TOPIC_NAME>"
  - name: AZURE_SERVICE_BUS_SUBSCRIPTION_NAME
    value: "<SUBSCRIPTION_NAME>"
  ```

  ```yaml
  # Configuration for AI
  - name: AZURE_OPENAI_API_VERSION
    value: "2024-07-01-preview"
  - name: AZURE_OPENAI_DEPLOYMENT_NAME
    value: "gpt-4-deployment"
  - name: AZURE_OPENAI_ENDPOINT
    value: "https://<your-openai-resource-name>.openai.azure.com/"
  - name: AZURE_OPENAI_DALLE_ENDPOINT
    value: "https://<your-openai-resource-name>.openai.azure.com/"
  - name: AZURE_OPENAI_DALLE_DEPLOYMENT_NAME
    value: "dalle-3-deployment"
  ```

  ```yaml
  # Configuration for Makeline
    - name: SERVICE_BUS_CONNECTION_STRING
    valueFrom:
      secretKeyRef:
        name: azure-service-bus-secrets2
        key: connectionString2
  - name: SERVICE_BUS_QUEUE_NAME
    value: orders
  - name: ORDER_QUEUE_URI
    value: 'amqps://BestBuyNamespace.servicebus.windows.net'
  - name: ORDER_QUEUE_USERNAME
    value: listener
  - name: ORDER_QUEUE_PASSWORD
    valueFrom:
      secretKeyRef:
        name: order-queue-secret
        key: ORDER_QUEUE_PASSWORD
  - name: ORDER_QUEUE_NAME
    value: orders
  - name: ORDER_DB_URI
    value: 'mongodb://mongodb:27017'
  - name: ORDER_DB_NAME
    value: orderdb
  - name: ORDER_DB_COLLECTION_NAME
    value: orders
  ```

### Task 7: Deploy the Secrets

#### Create and Deploy the Secret for OpenAI API:
- Make sure that you have replaced `Base64-encoded-API-KEY` in `secrets-orderService.yaml` and `secrets-AI.yaml` with your Base64-encoded OpenAI API key.

```sh
kubectl apply -f secrets-orderService.yaml
kubectl apply -f secrets-AI.yaml
kubectl apply -f secrets-makeline.yaml
```

#### Verify:
```sh
kubectl get secrets
```

- If you prefer not to store secrets in a YAML file, you can use this method in the CLI

<!-- connection string = service bus que > settings > shared access policies -->

```sh

# Order-service
kubectl create secret generic azure-servicebus-secret \
  --from-literal=connectionString="your-actual-connection-string" 

# AI-service
kubectl create secret generic openai-api-secret \
  --from-literal=OPENAI_API_KEY="your-actual-api-key"

# Makeline
kubectl create secret generic azure-service-bus-secrets2 \
  --from-literal=connectionString2="Endpoint=sb://example-servicebus.servicebus.windows.net/;SharedAccessKeyName=RootManageSharedAccessKey;SharedAccessKey=H8vN+9uL3qfJk73S9xQwKcM5kYZz0b2+3EdVQhXGZsY="

kubectl create secret generic order-queue-secret \
  --from-literal=ORDER_QUEUE_PASSWORD="P@ssw0rd!xA7#Ld29zKwQP8vNcU"
```

### Task 8: Deploy the Application
```sh
kubectl apply -f aps-all-in-one.yaml
```

#### Validate the Deployment
**Check Pods and Services:**  
```sh
kubectl get pods
kubectl get services
```

**Test Frontend Access:**  
- Locate the external IPs for `store-front` and `store-admin` services:
```sh
kubectl get services
```
- Access the **Store Front** app at the external IP on port `80`.
- Access the **Store Admin** app at the external IP on port `80`.

### Optional: Deploy Virtual Customer and Worker
```sh
kubectl apply -f admin-tasks.yaml
```

#### Monitor Virtual Customer:
```sh
kubectl logs -f deployment/virtual-customer
```

#### Monitor Virtual Worker:
```sh
kubectl logs -f deployment/virtual-worker
```

### Optional: Scale and Monitor Services
#### Scale Deployments:
**Scale the order-service to 3 replicas:**  
```sh
kubectl scale deployment order-service --replicas=3
```

#### Check Scaling:
```sh
kubectl get pods
```

#### Monitor Resource Usage:
- Enable metrics server for resource monitoring.
- Use `kubectl top` to monitor pod and node usage:
```sh
kubectl top pods
kubectl top nodes
```

## Table of Microservice Repositories
| Service        | Repository Link |
|---------------|----------------|
| Store-Front   | `https://github.com/degu0055/store-front-final`   |
| Product-Service   | `https://github.com/degu0055/product-service-final`   |
| Order-Service | `https://github.com/degu0055/order-service-final`   |
| Makeline | `https://github.com/degu0055/makeline-service-L8-final`   |
| Store Admin | `https://github.com/degu0055/store-admin-L8-final`   |


## Table of Docker Images
| Service        | Docker Image Link |
|---------------|------------------|
| Store-Front   | `https://hub.docker.com/repository/docker/degu0055/store-front-bestbuy` |
| Product-Service   | `https://hub.docker.com/repository/docker/degu0055/product-service-bestbuy/`   |
| Order-Service | `https://hub.docker.com/repository/docker/degu0055/order-service-bestbuy/` |
| Makeline | `https://hub.docker.com/repository/docker/degu0055/makeline-service-bestbuy/` |
| Store Admin | `https://hub.docker.com/repository/docker/degu0055/admin-bestbuy/` |


<!--  Uncomment this if found an answer
## Issues or Limitations (Optional)
Any issues or limitations in the implementation. -->

## Deployment Files Subfolder
 <!-- Include all Kubernetes deployment YAML files in a folder named `Deployment Files`.
Ensure these files are clearly named (e.g., `store-front-deployment.yaml`, `order-service-deployment.yaml`). -->
[Deployement Files](https://github.com/degu0055/25W_LabAssignment/tree/main/Deployment%20Files)


## Demo Video
<!--

Record a 5-minute max demo video showcasing the following:

- The application in action after deployment to AKS cluster.
- AI-generated product descriptions and images.
- Integration with the managed order queue service.

Upload the video to YouTube and include a link to the video in your README.md file under a "Demo Video" section. 

-->
> **Note:** The AI isn’t functioning in this demonstration, but I managed to get it to work prior to the assignment
> It is possible that the issue is due to quota limits, as there were days when I was troubleshooting the connection between Azure Service Bus, which I was unable to fix immediately.

[Lab Project Assignment](https://drive.google.com/drive/folders/1jhZCx8OWYEhi0KzdWlklff9rOgbELy74?usp=sharing)

[BackUp link](https://photos.app.goo.gl/frEPLkN7wYgxvsfA9)

## Reference
[Lab 6 - GitHub Repository](https://github.com/ramymohamed10/Lab6_25W_CST8915/blob/main/README.md)

[Lab 8 - GitHub Repository](https://github.com/ramymohamed10/Lab8_24F_CST8915)

[Order Service - GitHub Repository](https://github.com/ramymohamed10/order-service-L8)

[Makeline Service - Github Repository](https://github.com/ramymohamed10/makeline-service-L8)

<!-- [Others](https://github.com/ramymohamed10/algonquin-pet-store-on-steroids?tab=readme-ov-file) -->


