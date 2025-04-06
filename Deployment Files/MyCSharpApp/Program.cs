using Microsoft.Azure.ServiceBus;
using System;
using System.Text;
using System.Threading.Tasks;

class Program
{
    static string connectionString = "Endpoint=sb://bestbuynamespace.servicebus.windows.net/;SharedAccessKeyName=listener;SharedAccessKey=mn4cDI69Yoracp62QM7zd9OBguqM7V28f+ASbMy0blQ=";
    static string queueName = "orders";
    static IQueueClient queueClient;

    static async Task Main(string[] args)
    {
        queueClient = new QueueClient(connectionString, queueName);

        // Register the message handler
        queueClient.RegisterMessageHandler(
            async (message, token) =>
            {
                string body = Encoding.UTF8.GetString(message.Body);
                Console.WriteLine($"Received message: {body}");

                // Complete the message to remove it from the queue
                await queueClient.CompleteAsync(message.SystemProperties.LockToken);
            },
            new MessageHandlerOptions(ExceptionReceivedHandler) { MaxConcurrentCalls = 1, AutoComplete = false }
        );

        Console.ReadLine();  // Keep the program running
    }

    // Error handler for the message handler
    static Task ExceptionReceivedHandler(ExceptionReceivedEventArgs arg)
    {
        Console.WriteLine($"Exception: {arg.Exception.Message}");
        return Task.CompletedTask;
    }
}
