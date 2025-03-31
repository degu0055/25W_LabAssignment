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


