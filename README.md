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
### Prerequisite:
- Installed kubectl  
- Azure Kubernetes Cluster  

### Task 1: Set Up the AI Backing Services
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

### Task 2: Deploy the GPT-4 and DALL-E 3 Models
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

### Task 3: Retrieve and Configure API Keys
#### 1. Get API Keys:
- Go to the **Keys and Endpoints** section of your Azure OpenAI resource.
- Copy the **API Key (API key 1)** and **Endpoint URL**.  

#### 2. Base64 Encode the API Key:
```sh
echo -n "<your-api-key>" | base64
```
Replace `<your-api-key>` with your actual API key.

### Task 4: Update AI Service & Order Servive Deployment Configuration in the Deployment Files folder
#### 1. Modify Secrets YAML:
- Edit the `secrets.yaml` file.
- Replace `OPENAI_API_KEY` placeholder with the Base64-encoded value of the `API_KEY`.

#### 2. Modify Deployment YAML:
- Edit the `aps-all-in-one.yaml` file.
- Replace the placeholders with the configurations you retrieved:
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

### Task 5: Deploy the ConfigMaps and Secrets
#### Deploy the ConfigMap for RabbitMQ Plugins:
```sh
kubectl apply -f config-maps.yaml
```

#### Create and Deploy the Secret for OpenAI API:
- Make sure that you have replaced `Base64-encoded-API-KEY` in `secrets.yaml` with your Base64-encoded OpenAI API key.
```sh
kubectl apply -f secrets.yaml
```

#### Verify:
```sh
kubectl get configmaps
kubectl get secrets
```

### Task 6: Deploy the Application
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
| Store-Front   | `<GitHub Link>`   |
| Product-Service   | `<GitHub Link>`   |
| Order-Service | `<GitHub Link>`   |

## Table of Docker Images
| Service        | Docker Image Link |
|---------------|------------------|
| Store-Front   | `<Docker Hub Link>` |
| Product-Service   | `<Docker Hub Link>`   |
| Order-Service | `<Docker Hub Link>` |

<!--  Uncomment this if found an answer
## Issues or Limitations (Optional)
Any issues or limitations in the implementation. -->

## Deployment Files Subfolder
 <!-- Include all Kubernetes deployment YAML files in a folder named `Deployment Files`.
Ensure these files are clearly named (e.g., `store-front-deployment.yaml`, `order-service-deployment.yaml`). -->
Link: https://github.com/degu0055/25W_LabAssignment/tree/main/Deployment%20Files


## Demo Video
<!-- Record a 5-minute max demo video showcasing the following:

- The application in action after deployment to AKS cluster.
- AI-generated product descriptions and images.
- Integration with the managed order queue service.

Upload the video to YouTube and include a link to the video in your README.md file under a "Demo Video" section. -->


