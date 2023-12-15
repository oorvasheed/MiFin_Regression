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

WebUI.setText(findTestObject('Object Repository/all in one/Page_miFIN/input_miFIN_userId'), 'farzanan')

WebUI.setEncryptedText(findTestObject('Object Repository/all in one/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/all in one/Page_miFIN/button_LOGIN'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.maximizeWindow()

WebUI.click(findTestObject('Object Repository/all in one/Page_DASHBOARD/i_VIEWER_img1200004000'))

WebUI.click(findTestObject('Object Repository/all in one/Page_DASHBOARD/i_DM APPLICATION_img1200004003'))

WebUI.click(findTestObject('Object Repository/all in one/Page_DASHBOARD/a_DM SANCTION'))

WebUI.setText(findTestObject('Object Repository/all in one/Page_miFIN/input_Customer Name_firstName_1'), last_name)

WebUI.click(findTestObject('Object Repository/all in one/Page_miFIN/input_Delivery Date_search'))

WebUI.click(findTestObject('Object Repository/all in one/Page_miFIN/a_DMFIN1000008506'))

WebUI.waitForPageLoad(3)

WebUI.scrollToElement(findTestObject('Object Repository/all in one/Page_miFIN/select_SELECTHOLD REJECT APPROVE SEND BACK _efe445'), 
    0, FailureHandling.STOP_ON_FAILURE)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/Page_miFIN/select_SELECTHOLD REJECT APPROVE SEND BACK _efe445'), 
    '1000000004', true)

WebUI.setText(findTestObject('Object Repository/all in one/Page_miFIN/input_Remarks_remark_1_2'), 'ok')

WebUI.waitForElementClickable(findTestObject('Object Repository/all in one/Page_miFIN/a_Save'), 3, FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/all in one/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/all in one/Page_miFIN/img_Hi FARZANAN_userr'))

WebUI.click(findTestObject('Object Repository/all in one/Page_miFIN/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

