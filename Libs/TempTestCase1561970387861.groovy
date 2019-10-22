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


RunConfiguration.setExecutionSettingFile('C:\\Users\\kris\\AppData\\Local\\Temp\\Katalon\\20190701_163947\\execution.properties')

TestCaseMain.beforeStart()

        TestCaseMain.runWSVerificationScript(new TestCaseBinding('',[:]), 'import static org.assertj.core.api.Assertions.*\n\nimport com.kms.katalon.core.testobject.RequestObject\nimport com.kms.katalon.core.testobject.ResponseObject\nimport com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS\nimport com.kms.katalon.core.webservice.verification.WSResponseManager\n\nimport groovy.json.JsonSlurper\nimport internal.GlobalVariable as GlobalVariable\n\nRequestObject request = WSResponseManager.getInstance().getCurrentRequest()\n\nResponseObject response = WSResponseManager.getInstance().getCurrentResponse()\n\ndef jsonSlurper = new JsonSlurper()\n\ndef jsonResponse = jsonSlurper.parseText(response.getResponseText())\n\nif(jsonResponse.message == \'success\') {\n\n\tWS.verifyResponseStatusCode(response, 200)\n\t\n\tassertThat(response.getStatusCode()).isEqualTo(200)\n\t\n\tWS.verifyElementPropertyValue(response, \'data\', \'success\')\n\t\n\tWS.verifyElementPropertyValue(response, \'message\', \'success\')\n\n} else {\n\n\tWS.verifyResponseStatusCode(response, 400)\n\t\n\tassertThat(response.getStatusCode()).isEqualTo(400)\n\t\n\tWS.verifyElementPropertyValue(response, \'data\', \'[]\')\n\t\n\tWS.verifyElementPropertyValue(response, \'message\', \'Failed\')\n\n}', FailureHandling.STOP_ON_FAILURE, true)

