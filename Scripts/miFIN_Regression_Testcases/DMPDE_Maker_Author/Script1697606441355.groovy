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

WebUI.click(findTestObject('Object Repository/all in one/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Object Repository/all in one/a_REGISTRATION AND ASSET DETAILS'))

WebUI.click(findTestObject('Object Repository/all in one/input_REGISTRATION AND ASSET DETAILS_btn bt_566fe1'))

WebUI.click(findTestObject('Object Repository/all in one/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Object Repository/all in one/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/all in one/input_miFIN_userId'), 'farzanan')

WebUI.setEncryptedText(findTestObject('Object Repository/all in one/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/all in one/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/all in one/i_DM VIEWER_img1000000017'))

WebUI.click(findTestObject('Object Repository/all in one/i_INVOICE_img1200106858'))

WebUI.click(findTestObject('Object Repository/all in one/a_AUTHOR_1'))

WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_prospectNo_1'), DMCode)

WebUI.click(findTestObject('Object Repository/all in one/input_Revaluation Pending_search'))

WebUI.click(findTestObject('Object Repository/all in one/a_DMFIN1000008259_1'))

WebUI.click(findTestObject('Object Repository/all in one/a_REGISTRATION AND ASSET DETAILS'))

WebUI.click(findTestObject('Object Repository/all in one/input_Approval Decision ApproveReject_appDecision'))

WebUI.setText(findTestObject('Object Repository/all in one/input_Remarks_AUTHOR_REMARKS_ID_1_2'), 'ok')

WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/all in one/img_Hi FARZANAN_userr'))

WebUI.click(findTestObject('Object Repository/all in one/a_Logout'))

WebUI.comment('DM PD is approved, complete after that')

