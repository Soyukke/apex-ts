/**
 * Contact management class
 * 
 * @tsexport
 * @author Salesforce Team
 */
public class Contact {
    @AuraEnabled
    public String firstName;
    
    @AuraEnabled
    public String lastName;
    
    @AuraEnabled public String phone;  // One-line annotation style
    
    @AuraEnabled
    public Date birthDate;
    
    @AuraEnabled
    public List<Address> addresses;
    
    // This field will generate a warning
    public String secretField;
    
    @AuraEnabled
    public static Contact createContact(String firstName, String lastName) {
        return null;
    }
}
