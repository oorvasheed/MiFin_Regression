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

WebUI.setText(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/input_miFIN_userId'), 'copsuserm')

WebUI.setEncryptedText(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/button_LOGIN'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.maximizeWindow()

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_DASHBOARD/i_DM VIEWER_img1000000017'))

WebUI.scrollToElement(findTestObject('Rescheduling_Case/Page_DASHBOARD/a_RESCHEDULING CASE'), 0)

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_DASHBOARD/i_GENERATION_img1000001045'))

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_DASHBOARD/a_MAKER'))

WebUI.setText(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/input_National IDBRNPP No_generalFilter'), 
    nic_num)

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/input_Revaluation Pending_search'))

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/a_DMFIN1000008579'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/select_NOYES'), 'Y', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/select_--Select-- RESTRUCTUREPOSITIVE RESTR_c08366'), 
    '1000000001', true)

WebUI.setText(findTestObject('Rescheduling_Case/Page_miFIN/input__serviceTicked'), '9924')

String dateApp = WebUI.getText(findTestObject('Rescheduling_Case/Maker/Page_miFIN/span_22-AUG-2023'))

WebUI.setText(findTestObject('Rescheduling_Case/Page_miFIN/input__dateOfInvocation'), dateApp)

WebUI.selectOptionByIndex(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/select_--Select--01-AUG-2023'), 
    '1', FailureHandling.STOP_ON_FAILURE)

WebUI.selectOptionByValue(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/select_--Select-- RESTRUCTURING'), 
    '1000000055', true)

WebUI.scrollToElement(findTestObject('Rescheduling_Case/Page_miFIN/td_Excess Amount'), 0)

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/input_Installment Amount_prinRecadd'))

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/input_Installment Amount_prinRecadd'))

WebUI.setText(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/input_Installment Amount_noOfInstallmentsNew'), 
    '2')

WebUI.selectOptionByValue(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/select_COMPLETE RECOVERY WITH EMI FIXEDCOMP_d73f93'), 
    'EMI MORATORIUM', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/select_COMPLETE RECOVERY WITH EMI FIXEDCOMP_d73f93_1'), 
    'COMPLETE RECOVERY WITH EMI FIXED', true)

WebUI.setText(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/input_Installment Amount_installmentAmountNew'), 
    '500')

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/input__genReshSummaryReport'))

WebUI.switchToWindowTitle('miFIN')

WebUI.scrollToElement(findTestObject('Rescheduling_Case/Page_miFIN/span_Proposed'), 0)

WebUI.scrollToElement(findTestObject('Rescheduling_Case/Page_miFIN/legend_Existing Repayment Schedule'), 0)

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/input_Send To Author_mrSendToAuthor'))

WebUI.setText(findTestObject('Rescheduling_Case/Page_miFIN/textarea__mrRemarks'), 'ok')

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/input_miFIN_userId'), 'farzanan')

WebUI.setEncryptedText(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_DASHBOARD/i_DM VIEWER_img1000000017'))

WebUI.scrollToElement(findTestObject('Rescheduling_Case/Page_DASHBOARD/a_RESCHEDULING CASE'), 0)

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_DASHBOARD/i_GENERATION_img1000001045'))

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_DASHBOARD/a_AUTHOR'))

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/input_RESTRUCTURING_cbox'))

WebUI.setText(findTestObject('Rescheduling_Case/Page_miFIN/textarea_Author Remarks_authorRemarks'), 'ok')

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.delay(3, FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Rescheduling_Case/Page_miFIN/img_Hi FARZANAN_userr'))

WebUI.click(findTestObject('Object Repository/Rescheduling_Case/Page_miFIN/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

