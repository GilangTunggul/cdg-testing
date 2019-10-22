package internal

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.main.TestCaseMain


/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */
public class GlobalVariable {
     
    /**
     * <p></p>
     */
    public static Object url
     
    /**
     * <p></p>
     */
    public static Object token
     
    /**
     * <p></p>
     */
    public static Object cookies
     
    /**
     * <p></p>
     */
    public static Object tokenapp
     
    /**
     * <p></p>
     */
    public static Object cookiesspecialist
     
    /**
     * <p></p>
     */
    public static Object tokencsapp
     

    static {
        try {
            def selectedVariables = TestCaseMain.getGlobalVariables("default")
			selectedVariables += TestCaseMain.getGlobalVariables(RunConfiguration.getExecutionProfile())
            selectedVariables += RunConfiguration.getOverridingParameters()
    
            url = selectedVariables['url']
            token = selectedVariables['token']
            cookies = selectedVariables['cookies']
            tokenapp = selectedVariables['tokenapp']
            cookiesspecialist = selectedVariables['cookiesspecialist']
            tokencsapp = selectedVariables['tokencsapp']
            
        } catch (Exception e) {
            TestCaseMain.logGlobalVariableError(e)
        }
    }
}
