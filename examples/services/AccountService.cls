/**
 * Service layer for business logic
 * @tsexport
 */
public class AccountService {
    
    @AuraEnabled
    public static Map<String, Integer> getAccountStatistics() {
        Map<String, Integer> stats = new Map<String, Integer>();
        stats.put('totalAccounts', [SELECT COUNT() FROM Account]);
        stats.put('activeAccounts', [SELECT COUNT() FROM Account WHERE IsActive__c = true]);
        return stats;
    }
    
    @AuraEnabled
    public static List<Account> searchAccountsByIndustry(String industry, Integer maxResults) {
        String searchQuery = '%' + industry + '%';
        return [
            SELECT Id, Name, Industry, AnnualRevenue 
            FROM Account 
            WHERE Industry LIKE :searchQuery 
            LIMIT :maxResults
        ];
    }
    
    @AuraEnabled
    public static Boolean validateAccount(String accountId) {
        Account acc = [SELECT Id, Name, Email__c FROM Account WHERE Id = :accountId];
        return acc.Name != null && acc.Email__c != null;
    }
}
