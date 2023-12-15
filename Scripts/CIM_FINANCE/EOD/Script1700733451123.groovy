import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://mifinuat.cim.local/lease/')

WebUI.setText(findTestObject('Object Repository/EOD/Page_miFIN/input_miFIN_userId'), 'SUPERUSER')

WebUI.setEncryptedText(findTestObject('Object Repository/EOD/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/EOD/Page_miFIN/button_LOGIN'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/EOD/Page_DASHBOARD/i_LEASE RESIDUAL VALUE UPLOAD_img1100010049'))

WebUI.click(findTestObject('Object Repository/EOD/Page_DASHBOARD/a_EOD'))

WebUI.click(findTestObject('EOD/Page_miFIN/input_Email is Not in Records or not Update_master_1'))

WebUI.click(findTestObject('EOD/Page_miFIN/input_Email is Not in Records or not Update_logout'))

WebUI.scrollToElement(findTestObject('EOD/Page_miFIN/b_EODEOM Details Between Two Date'), 0)

WebUI.click(findTestObject('Object Repository/EOD/Page_miFIN/input_EODEOM Tracking_runEod1'))

WebUI.acceptAlert()

WebUI.waitForElementPresent(findTestObject('EOD/Page_miFIN/b_EOD Execution Start Date and Time'), 0)

WebUI.scrollToElement(findTestObject('EOD/Page_miFIN/b_EOD Execution Start Date and Time'), 0)

WebUI.delay(1390)

WebUI.acceptAlert()

System.out.println('EOD Completed')

WebUI.closeBrowser()

