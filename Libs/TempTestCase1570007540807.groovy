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


RunConfiguration.setExecutionSettingFile('C:\\Users\\HEWLET~1\\AppData\\Local\\Temp\\Katalon\\20191002_161220\\execution.properties')

TestCaseMain.beforeStart()

        TestCaseMain.runWSVerificationScript(new TestCaseBinding('',[:]), 'import static org.assertj.core.api.Assertions.*\n\nimport com.kms.katalon.core.testobject.RequestObject\nimport com.kms.katalon.core.testobject.ResponseObject\nimport com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS\nimport com.kms.katalon.core.webservice.verification.WSResponseManager\n\nimport groovy.json.JsonSlurper\nimport internal.GlobalVariable as GlobalVariable\n\nRequestObject request = WSResponseManager.getInstance().getCurrentRequest()\n\nResponseObject response = WSResponseManager.getInstance().getCurrentResponse()\n\nWS.verifyResponseStatusCode(response, 200)\n\nassertThat(response.getStatusCode()).isEqualTo(200)\n\ndef jsonSlurper = new JsonSlurper()\n\ndef jsonResponse = jsonSlurper.parseText(response.getResponseText())\n\nJsonSlurper slurper = new JsonSlurper()\n\ndef parsedJson = slurper.parseText(response.getResponseText())\n\nprintln(jsonResponse.data)\n\nif (jsonResponse.data == [] ) {\n\t\n\tWS.verifyElementPropertyValue(response, \'success\', \'true\')\n\t\n\tWS.verifyElementPropertyValue(response, \'data\', \'[]\')\n\t\n} else {\n\t\n\tWS.verifyElementPropertyValue(response, \'message\', \'[]\')\n\n\tWS.verifyElementPropertyValue(response, \'data\', jsonResponse.data)\n\t\n\tWS.verifyElementPropertyValue(response, \'success\', \'true\')\n\t\n\tdef expectedSize = jsonResponse.data.data.size()\n\t\n\tprintln(expectedSize)\n\t\n\tfor(int i = 0; i < expectedSize; i++) {\n\t\n\t\tWS.verifyElementPropertyValue(response, \'data.data\' + [i] + \'.customerID\', \'5d142a33924e18084fdfd885\')\n\t\t\n\t\tWS.verifyElementPropertyValue(response, \'data.data\' + [i] + \'.companyID\', \'5948f33556d6c91479154da2\')\n\t\n\t}\n\t\n\tString a = parsedJson.data.data.id\n\t\n\tString b = parsedJson.data.data.productNumber\n\t\n\tString c = parsedJson.data.data.ticketNumber\n\t\n\tString[] arrayResponse = [a, b, c]\n\t\n\tassertThat(arrayResponse).containsOnly(a, b, c)\n\t\n\tassertThat(arrayResponse).containsOnlyElementsOf(Arrays.asList(a, b, c))\n\t\n}\n', FailureHandling.STOP_ON_FAILURE, true)

