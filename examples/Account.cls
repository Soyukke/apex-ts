/**
 * Account class for managing customer accounts
 * @tsexport
 * @description This class handles account operations
 */
public class Account {
    @AuraEnabled
    public String name;
    
    @AuraEnabled
    public String email;
    
    @AuraEnabled public Integer age;  // One-line annotation
    
    @AuraEnabled
    public Boolean isActive;
    
    @AuraEnabled
    public List<String> tags;
    
    @AuraEnabled
    public Map<String, Integer> scores;
    
    // This field doesn't have @AuraEnabled, so it will generate a warning
    public String internalId;
    
    @AuraEnabled(cacheable=true)
    public static Account getAccountById(String accountId) {
        return null;
    }
    
    @AuraEnabled
    public static List<Account> searchAccounts(String query, Integer maxResults) {
        return null;
    }
    
    @AuraEnabled public void updateEmail(String newEmail) {  // One-line annotation
        // implementation
    }
    
    // This method doesn't have @AuraEnabled, so it will generate a warning
    public void internalMethod() {
        // implementation
    }
}
