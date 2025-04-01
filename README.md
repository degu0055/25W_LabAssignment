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
Installed kubectl  
Azure Kubernetes Cluster  
AI Backing Services  

### Task 1: Retrieve and Configure API Keys

1. Get API Keys:
    * Go to the Keys and Endpoints section of your Azure OpenAI resource.
    * Copy the API Key (API key 1) and Endpoint URL.
2. Base64 Encode the API Key:
    ```sh
echo -n "<your-api-key>" | base64  
    ```
    * Replace `<your-api-key>` with your actual API key.

### Task 2: Update AI Service Deployment Configuration in the Deployment Files folder.

1. Modify Secretes YAML:
    * Edit the `secrets.yaml` file.
    * Replace `OPENAI_API_KEY` placeholder with the Base64-encoded value of the API_KEY.
2. Modify Deployment YAML:
    * Edit the `aps-all-in-one.yaml` file.
    * Replace the placeholders with the configurations you retrieved:
        * `AZURE_OPENAI_DEPLOYMENT_NAME`: Enter the deployment name for GPT-4.
        * `AZURE_OPENAI_ENDPOINT`: Enter the endpoint URL for the GPT-4 deployment.
        * `AZURE_OPENAI_DALLE_ENDPOINT`: Enter the endpoint URL for the DALL-E 3 deployment.
        * `AZURE_OPENAI_DALLE_DEPLOYMENT_NAME`: Enter the deployment name for DALL-E 3.
3. Example configuration in the YAML file:
    ```yaml
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

### Task 3: Deploy the ConfigMaps and Secrets

* Deploy the ConfigMap for RabbitMQ Plugins:
    ```sh
    kubectl apply -f config-maps.yaml  
    ```
* Create and Deploy the Secret for OpenAI API:
    * Make sure that you have replaced `Base64-encoded-API-KEY` in `secrets.yaml` with your Base64-encoded OpenAI API key.
* Deploy the secret:
    ```sh
    kubectl apply -f secrets.yaml  
    ```
* Verify:
    ```sh
    kubectl get configmaps
    kubectl get secrets  
    ```

### Task 4: Deploy the Application

```sh
kubectl apply -f aps-all-in-one.yaml
```

### Validate the Deployment

* Check Pods and Services:
    ```sh
    kubectl get pods
    kubectl get services  
    ```
* Test Frontend Access:
    * Locate the external IPs for `store-front` and `store-admin` services:
    ```sh
    kubectl get services  
    ```
    * Access the Store Front app at the external IP on port 80.
    * Access the Store Admin app at the external IP on port 80.

### Task 5: Deploy Virtual Customer and Worker

```sh
kubectl apply -f admin-tasks.yaml
```

* Monitor Virtual Customer:
    ```sh
    kubectl logs -f deployment/virtual-customer  
    ```
* Monitor Virtual Worker:
    ```sh
    kubectl logs -f deployment/virtual-worker  
    ```

### Optional: Scale and Monitor Services

#### Scale Deployments:

* Scale the `order-service` to 3 replicas:
    ```sh
    kubectl scale deployment order-service --replicas=3
    ```
* Check Scaling:
    ```sh
    kubectl get pods
    ```

#### Monitor Resource Usage:

* Enable metrics server for resource monitoring.
* Use `kubectl top` to monitor pod and node usage:
    ```sh
    kubectl top pods
    kubectl top nodes  
    

## Table of Microservice Repositories
| Service        | Repository Link |
|---------------|----------------|
| Store-Front   | `<GitHub Link>`   |
| Order-Service | `<GitHub Link>`   |

## Table of Docker Images
| Service        | Docker Image Link |
|---------------|------------------|
| Store-Front   | `<Docker Hub Link>` |
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


