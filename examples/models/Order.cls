/**
 * Order model
 * @tsexport
 */
public class Order {
    @AuraEnabled
    public String orderId;
    
    @AuraEnabled
    public String accountId;
    
    @AuraEnabled
    public Date orderDate;
    
    @AuraEnabled
    public Decimal totalAmount;
    
    @AuraEnabled
    public String status;
    
    @AuraEnabled
    public List<Product> items;
    
    @AuraEnabled(cacheable=true)
    public static Order getOrderById(String orderId) {
        // Implementation
        return null;
    }
    
    @AuraEnabled
    public static List<Order> getOrdersByAccount(String accountId, Integer maxRecords) {
        // Implementation
        return null;
    }
    
    @AuraEnabled
    public static Order createOrder(String accountId, Decimal totalAmount, List<Product> items) {
        // Implementation
        return null;
    }
    
    @AuraEnabled
    public static void updateOrderStatus(String orderId, String newStatus) {
        // Implementation
    }
}
