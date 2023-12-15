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

WebUI.setText(findTestObject('Object Repository/all in one/input_miFIN_userId'), 'farzanan')

WebUI.setEncryptedText(findTestObject('Object Repository/all in one/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/all in one/button_LOGIN'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.maximizeWindow()

WebUI.click(findTestObject('Object Repository/all in one/i_DM VIEWER_img1000000017'))

WebUI.click(findTestObject('Object Repository/all in one/i_MAKER_img1000000034'))

WebUI.click(findTestObject('Object Repository/all in one/Page_DASHBOARD/a_AUTHOR'))

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_National IDBRNPP No_generalFilter (1)'), nic_num)

WebUI.click(findTestObject('Object Repository/all in one/input_Revaluation Pending_search'))

WebUI.click(findTestObject('Object Repository/all in one/a_DMFIN1000008259_1_2'))

WebUI.scrollToElement(findTestObject('Object Repository/Page_miFIN/input_Approve_arApprove'), 0)

WebUI.click(findTestObject('Object Repository/Page_miFIN/input_Approve_arApprove'))

WebUI.setText(findTestObject('Object Repository/Page_miFIN/textarea_Remarks_arRemarks_1'), 'ok')

WebUI.click(findTestObject('Object Repository/all in one/a_Save_1_2'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

WebUI.click(findTestObject('Receipt Approval/Page_miFIN/img_Hi FARZANAN_userr'))

WebUI.click(findTestObject('Receipt Approval/Page_miFIN/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

