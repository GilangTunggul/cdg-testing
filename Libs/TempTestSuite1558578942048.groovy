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


suiteProperties.put('id', 'Test Suites/Customer web/Test_Mybooking')

suiteProperties.put('name', 'Test_Mybooking')

suiteProperties.put('description', '')
 

DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.webui.contribution.WebUiDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.mobile.contribution.MobileDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.cucumber.keyword.internal.CucumberDriverCleaner())



RunConfiguration.setExecutionSettingFile("H:\\Katalon Studio\\API\\CDG - test\\Reports\\Customer web\\Test_Mybooking\\20190523_103541\\execution.properties")

TestCaseMain.beforeStart()

TestCaseMain.startTestSuite('Test Suites/Customer web/Test_Mybooking', suiteProperties, [new TestCaseBinding('Test Cases/Customer web/My booking/Testcase_Addschedule - Iteration 1', 'Test Cases/Customer web/My booking/Testcase_Addschedule',  [ 'ticketid' : '' , 'type' : 'cs' ,  ]), new TestCaseBinding('Test Cases/Customer web/My booking/Testcase_Addschedule - Iteration 2', 'Test Cases/Customer web/My booking/Testcase_Addschedule',  [ 'ticketid' : '5ce36cb2c7c95d0f554d9bf5' , 'type' : 'submit' ,  ]), new TestCaseBinding('Test Cases/Customer web/My booking/Testcase_Addschedule - Iteration 3', 'Test Cases/Customer web/My booking/Testcase_Addschedule',  [ 'ticketid' : '5ce36cb2c7c95d0f554d9bf5' , 'type' : 'pickup' ,  ]), new TestCaseBinding('Test Cases/Customer web/My booking/Testcase_Storeoption', 'Test Cases/Customer web/My booking/Testcase_Storeoption',  null), new TestCaseBinding('Test Cases/Customer web/My booking/Testcase_Timeoption', 'Test Cases/Customer web/My booking/Testcase_Timeoption',  null), new TestCaseBinding('Test Cases/Customer web/My booking/Testcase_Historyschedule', 'Test Cases/Customer web/My booking/Testcase_Historyschedule',  null), new TestCaseBinding('Test Cases/Customer web/My booking/Testcase_Historyschedule', 'Test Cases/Customer web/My booking/Testcase_Historyschedule',  null)])
