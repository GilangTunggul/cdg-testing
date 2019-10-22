import com.kms.katalon.core.logging.KeywordLogger
import com.kms.katalon.core.exception.StepFailedException
import com.kms.katalon.core.reporting.ReportUtil
import com.kms.katalon.core.main.TestCaseMain
import com.kms.katalon.core.testdata.TestDataColumn
import com.kms.katalon.core.testcase.TestCaseBinding
import com.kms.katalon.core.driver.internal.DriverCleanerCollector
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.configuration.RunConfiguration
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import internal.GlobalVariable as GlobalVariable

Map<String, String> suiteProperties = new HashMap<String, String>();


suiteProperties.put('id', 'Test Suites/Customer web/Test_Myticket')

suiteProperties.put('name', 'Test_Myticket')

suiteProperties.put('description', '')
 

DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.webui.contribution.WebUiDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.mobile.contribution.MobileDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.cucumber.keyword.internal.CucumberDriverCleaner())



RunConfiguration.setExecutionSettingFile("H:\\Katalon Studio\\API\\CDG - test\\Reports\\Customer web\\Test_Myticket\\20190521_150946\\execution.properties")

TestCaseMain.beforeStart()

TestCaseMain.startTestSuite('Test Suites/Customer web/Test_Myticket', suiteProperties, [new TestCaseBinding('Test Cases/Customer web/My ticket/Tescase_Ticketlist', 'Test Cases/Customer web/My ticket/Tescase_Ticketlist',  null), new TestCaseBinding('Test Cases/Customer web/My ticket/Testcase_Ticketdetail', 'Test Cases/Customer web/My ticket/Testcase_Ticketdetail',  null), new TestCaseBinding('Test Cases/Customer web/My ticket/Testcase_Trackingprogress', 'Test Cases/Customer web/My ticket/Testcase_Trackingprogress',  null), new TestCaseBinding('Test Cases/Customer web/My ticket/Testcase_Categories', 'Test Cases/Customer web/My ticket/Testcase_Categories',  null), new TestCaseBinding('Test Cases/Customer web/My ticket/Testcase_Selection', 'Test Cases/Customer web/My ticket/Testcase_Selection',  null), new TestCaseBinding('Test Cases/Customer web/My ticket/Testcase_Submitdevice', 'Test Cases/Customer web/My ticket/Testcase_Submitdevice',  null)])
