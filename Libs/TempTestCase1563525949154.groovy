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


RunConfiguration.setExecutionSettingFile('C:\\Users\\kris\\AppData\\Local\\Temp\\Katalon\\20190719_164549\\execution.properties')

TestCaseMain.beforeStart()

        TestCaseMain.runWSVerificationScript(new TestCaseBinding('',[:]), 'import static org.assertj.core.api.Assertions.*\n\nimport com.kms.katalon.core.testobject.RequestObject\nimport com.kms.katalon.core.testobject.ResponseObject\nimport com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS\nimport com.kms.katalon.core.webservice.verification.WSResponseManager\n\nimport groovy.json.JsonSlurper\nimport internal.GlobalVariable as GlobalVariable\n\nRequestObject request = WSResponseManager.getInstance().getCurrentRequest()\n\nResponseObject response = WSResponseManager.getInstance().getCurrentResponse()\n\nWS.verifyResponseStatusCode(response, 200)\n\nassertThat(response.getStatusCode()).isEqualTo(200)\n\ndef jsonSlurper = new JsonSlurper()\n\ndef jsonResponse = jsonSlurper.parseText(response.getResponseText())\n\nJsonSlurper slurper = new JsonSlurper()\n\ndef parsedJson = slurper.parseText(response.getResponseText())\n\nprintln(jsonResponse.data != null)\n\nWS.verifyElementPropertyValue(response, \'message\', \'[]\')\n\nWS.verifyElementPropertyValue(response, \'data\', jsonResponse.data)\n\nWS.verifyElementPropertyValue(response, \'success\', \'true\')\n\nString a = parsedJson.data.data.companyID\n\nString[] arrayResponse = [a]\n\nassertThat(arrayResponse).containsOnly(a)\n\nassertThat(arrayResponse).containsOnlyElementsOf(Arrays.asList(a))', FailureHandling.STOP_ON_FAILURE, true)

