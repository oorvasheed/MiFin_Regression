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

WebUI.setText(findTestObject('Object Repository/all in one/input_miFIN_userId'), 'COPSUSERM')

WebUI.setEncryptedText(findTestObject('Object Repository/all in one/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/all in one/button_LOGIN'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Post Disbursal_Doc/Page_DASHBOARD/i_VIEWER_img1200004000'))

WebUI.click(findTestObject('Post Disbursal_Doc/Page_DASHBOARD/a_DM VIEWER'))

WebUI.setText(findTestObject('Post Disbursal_Doc/Page_miFIN/input_National IDBRNPP No_generalFilter (1)'), nic_num)

WebUI.click(findTestObject('Post Disbursal_Doc/Page_miFIN/input_Engine No_search'))

String dmCode1 = WebUI.getText(findTestObject('Post Disbursal_Doc/Page_miFIN/a_DMFIN1000008514'))

WebUI.click(findTestObject('Object Repository/all in one/i_DM VIEWER_img1000000017'))

WebUI.click(findTestObject('Object Repository/all in one/i_INVOICE_img1200106858'))

WebUI.click(findTestObject('Object Repository/all in one/a_MAKER_1'))

/* to change below, to search by name*/
WebUI.setText(findTestObject('Post Disbursal_Doc/Page_miFIN/input_DM Code_prospectNo'), dmCode1)

//WebUI.setText(findTestObject('Post Disbursal_Doc/Page_miFIN/input_National IDBRNPP No_generalFilter'), nic_num)
WebUI.click(findTestObject('Object Repository/all in one/input_Revaluation Pending_search'))

WebUI.click(findTestObject('Object Repository/all in one/a_DMFIN1000008259_1'))

WebUI.click(findTestObject('Object Repository/all in one/a_DOCUMENT_1'))

WebUI.click(findTestObject('Object Repository/all in one/a_Post Disbursal'))

WebUI.click(findTestObject('Object Repository/all in one/input_Last Updated Date_UPDATED_CHK0'))

WebUI.click(findTestObject('Object Repository/all in one/td'))

WebUI.click(findTestObject('Object Repository/all in one/input_SYSUSER_UPDATED_CHK1'))

WebUI.click(findTestObject('Object Repository/all in one/input_SYSUSER_UPDATED_CHK2'))

WebUI.click(findTestObject('Object Repository/all in one/input_SYSUSER_UPDATED_CHK3'))

WebUI.click(findTestObject('Object Repository/all in one/input_SYSUSER_UPDATED_CHK4'))

WebUI.click(findTestObject('Object Repository/all in one/input_SYSUSER_UPDATED_CHK5'))

WebUI.click(findTestObject('Object Repository/all in one/input_SYSUSER_UPDATED_CHK6'), FailureHandling.OPTIONAL)

WebUI.setText(findTestObject('Object Repository/all in one/input_Last Updated Date_DOCUMENT_DESCRIPTION0'), 'ok')

WebUI.click(findTestObject('Object Repository/all in one/tr_SelectLESSEEUSERCO-LESSEEGUARANTORBUYERB_adcbee'))

WebUI.setText(findTestObject('Object Repository/all in one/input_Last Updated Date_DOCUMENT_DESCRIPTION0'), 'ok')

WebUI.setText(findTestObject('Object Repository/all in one/input_SYSUSER_DOCUMENT_DESCRIPTION1'), 'OK')

WebUI.setText(findTestObject('Object Repository/all in one/input_SYSUSER_DOCUMENT_DESCRIPTION2'), 'v')

WebUI.click(findTestObject('Object Repository/all in one/tr_SelectLESSEEUSERCO-LESSEEGUARANTORBUYERB_adcbee'))

WebUI.setText(findTestObject('Object Repository/all in one/input_SYSUSER_DOCUMENT_DESCRIPTION2'), 'OK')

WebUI.setText(findTestObject('Object Repository/all in one/input_SYSUSER_DOCUMENT_DESCRIPTION3'), 'OK')

WebUI.setText(findTestObject('Object Repository/all in one/input_SYSUSER_DOCUMENT_DESCRIPTION4'), 'OK')

WebUI.setText(findTestObject('Object Repository/all in one/input_SYSUSER_DOCUMENT_DESCRIPTION5'), 'OK')

WebUI.setText(findTestObject('Object Repository/all in one/input_SYSUSER_DOCUMENT_DESCRIPTION6'), 'OK', FailureHandling.OPTIONAL)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SelectRECEIVEDPENDINGDEFERRED'), '1000000001', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SelectRECEIVEDPENDINGDEFERRED_1'), '1000000001', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SelectRECEIVEDPENDINGDEFERRED_1_2'), '1000000001', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SelectRECEIVEDPENDINGDEFERRED_1_2_3'), '1000000001', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SelectRECEIVEDPENDINGDEFERRED_1_2_3_4'), '1000000001', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SelectRECEIVEDPENDINGDEFERRED_1_2_3_4_5'), 
    '1000000001', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SelectRECEIVEDPENDINGDEFERRED_1_2_3_4_5_6'), 
    '1000000001', true, FailureHandling.OPTIONAL)

WebUI.uploadFile(findTestObject('Post Disbursal_Doc/input_Last Updated Date_file'), file_kyc)

WebUI.uploadFile(findTestObject('Post Disbursal_Doc/input_SYSUSER_file'), file_kyc)

WebUI.uploadFile(findTestObject('Post Disbursal_Doc/input_SYSUSER_file(2)'), file_kyc)

WebUI.uploadFile(findTestObject('Post Disbursal_Doc/input_SYSUSER_file(3)'), file_kyc)

WebUI.uploadFile(findTestObject('Post Disbursal_Doc/input_SYSUSER_file(4)'), file_kyc)

WebUI.uploadFile(findTestObject('Post Disbursal_Doc/input_SYSUSER_file(5)'), file_kyc)

WebUI.uploadFile(findTestObject('Post Disbursal_Doc/input_SYSUSER_file(6)'), file_kyc, FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Post Disbursal_Doc/Page_miFIN/a_Save_17'))

WebUI.waitForAlert(2, FailureHandling.OPTIONAL)

WebUI.acceptAlert(FailureHandling.STOP_ON_FAILURE)

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Post Disbursal_Doc/Page_miFIN/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Post Disbursal_Doc/Page_miFIN/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

