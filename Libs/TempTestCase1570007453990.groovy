import com.kms.katalon.core.main.TestCaseMain
import com.kms.katalon.core.logging.KeywordLogger
import com.kms.katalon.core.testcase.TestCaseBinding
import com.kms.katalon.core.driver.internal.DriverCleanerCollector
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.webui.contribution.WebUiDriverCleaner
import com.kms.katalon.core.mobile.contribution.MobileDriverCleaner
import com.kms.katalon.core.cucumber.keyword.internal.CucumberDriverCleaner


DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.webui.contribution.WebUiDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.mobile.contribution.MobileDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.cucumber.keyword.internal.CucumberDriverCleaner())


RunConfiguration.setExecutionSettingFile('C:\\Users\\HEWLET~1\\AppData\\Local\\Temp\\Katalon\\20191002_161053\\execution.properties')

TestCaseMain.beforeStart()

        TestCaseMain.runWSVerificationScript(new TestCaseBinding('',[:]), 'import static org.assertj.core.api.Assertions.*\r\n\r\nRequestObject request = WSResponseManager.getInstance().getCurrentRequest()\r\n\r\nResponseObject response = WSResponseManager.getInstance().getCurrentResponse()\r\n\r\nWS.verifyResponseStatusCode(response, 200)\r\n\r\nassertThat(response.getStatusCode()).isEqualTo(200)\r\n\r\ndef jsonSlurper = new JsonSlurper()\r\n\r\ndef jsonResponse = jsonSlurper.parseText(response.getResponseText())\r\n\r\nJsonSlurper slurper = new JsonSlurper()\r\n\r\ndef parsedJson = slurper.parseText(response.getResponseText())\r\n\r\nprintln(jsonResponse.data)\r\n\r\nif (jsonResponse.data == [] ) {\r\n\t\r\n\tWS.verifyElementPropertyValue(response, \'success\', \'true\')\r\n\t\r\n\tWS.verifyElementPropertyValue(response, \'data\', \'[]\')\r\n\t\r\n} else {\r\n\t\r\n\tWS.verifyElementPropertyValue(response, \'message\', \'[]\')\r\n\r\n\tWS.verifyElementPropertyValue(response, \'data\', jsonResponse.data)\r\n\t\r\n\tWS.verifyElementPropertyValue(response, \'success\', \'true\')\r\n\t\r\n\tdef expectedSize = jsonResponse.data.data.size()\r\n\t\r\n\tprintln(expectedSize)\r\n\t\r\n\tfor(int i = 0; i < expectedSize; i++) {\r\n\t\r\n\t\tWS.verifyElementPropertyValue(response, \'data.data\' + [i] + \'.customerID\', \'5d142a33924e18084fdfd885\')\r\n\t\t\r\n\t\tWS.verifyElementPropertyValue(response, \'data.data\' + [i] + \'.companyID\', \'5948f33556d6c91479154da2\')\r\n\t\r\n\t}\r\n\t\r\n\tString a = parsedJson.data.data.id\r\n\t\r\n\tString b = parsedJson.data.data.productNumber\r\n\t\r\n\tString c = parsedJson.data.data.ticketNumber\r\n\t\r\n\tString[] arrayResponse = [a, b, c]\r\n\t\r\n\tassertThat(arrayResponse).containsOnly(a, b, c)\r\n\t\r\n\tassertThat(arrayResponse).containsOnlyElementsOf(Arrays.asList(a, b, c))\r\n\t\r\n}\r\n\r\n\r\n', FailureHandling.STOP_ON_FAILURE, true)

