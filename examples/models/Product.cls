/**
 * Product model
 * @tsexport
 */
public class Product {
    @AuraEnabled
    public String id;
    
    @AuraEnabled
    public String name;
    
    @AuraEnabled
    public Decimal price;
    
    @AuraEnabled
    public String description;
    
    @AuraEnabled
    public Boolean isAvailable;
    
    @AuraEnabled
    public List<String> categories;
    
    @AuraEnabled
    public Map<String, String> attributes;
    
    public String internalSku;  // Not exported (no @AuraEnabled)
}
