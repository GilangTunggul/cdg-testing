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


suiteProperties.put('id', 'Test Suites/Customer App/Test_Device')

suiteProperties.put('name', 'Test_Device')

suiteProperties.put('description', '')
 

DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.webui.contribution.WebUiDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.mobile.contribution.MobileDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.cucumber.keyword.internal.CucumberDriverCleaner())



RunConfiguration.setExecutionSettingFile("H:\\Katalon Studio\\API\\CDG - test\\Reports\\Customer App\\Test_Device\\20190523_155141\\execution.properties")

TestCaseMain.beforeStart()

TestCaseMain.startTestSuite('Test Suites/Customer App/Test_Device', suiteProperties, [new TestCaseBinding('Test Cases/Customer/App/Device/Testcase_Productselection', 'Test Cases/Customer/App/Device/Testcase_Productselection',  null), new TestCaseBinding('Test Cases/Customer/App/Device/Testcase_Adddevice', 'Test Cases/Customer/App/Device/Testcase_Adddevice',  null), new TestCaseBinding('Test Cases/Customer/App/Device/Testcase_Devicelist', 'Test Cases/Customer/App/Device/Testcase_Devicelist',  null), new TestCaseBinding('Test Cases/Customer/App/Device/Testcase_Devicedetail', 'Test Cases/Customer/App/Device/Testcase_Devicedetail',  null)])
