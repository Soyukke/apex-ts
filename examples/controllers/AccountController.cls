/**
 * Controller for Account operations
 * @tsexport
 */
public class AccountController {
    
    @AuraEnabled(cacheable=true)
    public static List<Account> getAccounts(Integer maxRecords) {
        return [SELECT Id, Name, Email FROM Account LIMIT :maxRecords];
    }
    
    @AuraEnabled
    public static Account createAccount(String name, String email, String phone) {
        Account acc = new Account(
            Name = name,
            Email__c = email,
            Phone = phone
        );
        insert acc;
        return acc;
    }
    
    @AuraEnabled
    public static void deleteAccount(String accountId) {
        Account acc = [SELECT Id FROM Account WHERE Id = :accountId];
        delete acc;
    }
    
    @AuraEnabled(cacheable=true)
    public static Account getAccountWithContacts(String accountId) {
        return [
            SELECT Id, Name, Email, 
                   (SELECT Id, FirstName, LastName FROM Contacts)
            FROM Account 
            WHERE Id = :accountId
        ];
    }
}
