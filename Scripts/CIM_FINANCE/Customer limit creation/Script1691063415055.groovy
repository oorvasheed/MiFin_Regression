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
import com.kms.katalon.core.webui.keyword.internal.WebUIAbstractKeyword as WebUIAbstractKeyword
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://mifinuat.cim.local/lease/')

WebUI.setText(findTestObject('Object Repository/New customer limit/input_miFIN_userId'), 'navind')

WebUI.setEncryptedText(findTestObject('Object Repository/New customer limit/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/New customer limit/button_LOGIN'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.maximizeWindow()

WebUI.click(findTestObject('Object Repository/New customer limit/i_GROUP EDIT_img1200006000'))

WebUI.click(findTestObject('Object Repository/New customer limit/i_GROUP LIMIT VIEWER_img1200007001'))

WebUI.click(findTestObject('Object Repository/New customer limit/i_CUSTOMER LIMIT_img1200007002'))

WebUI.click(findTestObject('Object Repository/New customer limit/a_CUSTOMER LIMIT'))

WebUI.click(findTestObject('Object Repository/New customer limit/input_Customer Code_blueBotton'))

WebUI.switchToWindowTitle('miFIN')

WebUI.closeWindowTitle('miFIN')

WebUI.switchToWindowUrl('https://mifinuat.cim.local/lease/customerLimit.do?actionPerformed=displaySearchScreen&searchType=CUSTOMER%20LIMIT&screenFlag=N&parentBodyId=customerLimitId')

WebUI.setText(findTestObject('Object Repository/New customer limit/input_LAST NAME_lastName'), last_name)

/* New changes end here */
WebUI.click(findTestObject('Object Repository/New customer limit/input_LIMIT CODE_blueBotton'))

WebUI.click(findTestObject('Object Repository/New customer limit/input_MOBILE_selectApplicant'))

WebUI.click(findTestObject('Object Repository/New customer limit/input_TESTING TEST_blueBotton'))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/New customer limit/input_Customer Code_blueBotton_1'))

WebUI.selectOptionByValue(findTestObject('Object Repository/New customer limit/select_SELECTCUSTOMERCVCVDEFAULT SCHEMEDEFA_18fdec'), 
    '1200000112', true)

WebUI.sendKeys(findTestObject('Object Repository/New customer limit/select_SELECTEQUIPMENT VEHICLE'), 'Vehicle')

WebUI.selectOptionByValue(findTestObject('Object Repository/New customer limit/select_SELECTREVOLVINGNON-REVOLVING'), 'N', 
    true)

WebUI.doubleClick(findTestObject('Object Repository/New customer limit/input_Requested Limit Amt_requestedLimitAmount'))

WebUI.sendKeys(findTestObject('Object Repository/New customer limit/input_Requested Limit Amt_requestedLimitAmount_1_2_3_4_5'), 
    '20000')

WebUI.selectOptionByValue(findTestObject('Object Repository/New customer limit/select_SELECTBOOK VALUEDISCOUNTING WITH RVD_2f199d'), 
    '1000000001', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/New customer limit/select_SELECTSTANDING ORDERDIRECT DEBITCASH_c321de'), 
    '1000000004', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/New customer limit/select_SELECT360'), '1000000001', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/New customer limit/select_SELECTDISCLOSEDUNDISCLOSED'), 'DISCLOSED', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/New customer limit/select_SELECTUSDGBPEURMUR'), '1000000001', 
    true)

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_penalty_installmentTo'), '120')

WebUI.click(findTestObject('New customer limit/a_Save  Continue'))

WebUI.verifyAlertPresent(2)

WebUI.acceptAlert()

WebUI.setText(findTestObject('Object Repository/New customer limit/input_Expiry Date_expiryDate'), '20-Jun-2025')

WebUI.click(findTestObject('New customer limit/a_Save  Continue'))

WebUI.acceptAlert()

WebUI.waitForAlert(5, FailureHandling.OPTIONAL)

WebUI.verifyAlertPresent(5, FailureHandling.OPTIONAL)

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Page_miFIN/a_Save'))

WebUI.acceptAlert()

/* Documents starts here */
WebUI.click(findTestObject('Object Repository/New customer limit/a_DOCUMENT'))

WebUI.click(findTestObject('Object Repository/New customer limit/input_Last Updated Date_UPDATED_CHK0'))

WebUI.click(findTestObject('Object Repository/New customer limit/input_SYSUSER_UPDATED_CHK1'))

WebUI.click(findTestObject('Object Repository/New customer limit/input_SYSUSER_UPDATED_CHK2'))

WebUI.click(findTestObject('Object Repository/New customer limit/input_SYSUSER_UPDATED_CHK3'))

WebUI.setText(findTestObject('Object Repository/New customer limit/input_Last Updated Date_DOCUMENT_DESCRIPTION0'), 'ok')

WebUI.setText(findTestObject('Object Repository/New customer limit/input_SYSUSER_DOCUMENT_DESCRIPTION1'), 'ok')

WebUI.setText(findTestObject('Object Repository/New customer limit/input_SYSUSER_DOCUMENT_DESCRIPTION2'), 'ok')

WebUI.setText(findTestObject('Object Repository/New customer limit/input_SYSUSER_DOCUMENT_DESCRIPTION3'), 'ok')

WebUI.selectOptionByValue(findTestObject('Object Repository/New customer limit/select_SelectRECEIVEDPENDINGDEFERRED'), '1000000001', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/New customer limit/select_SelectRECEIVEDPENDINGDEFERRED_1'), 
    '1000000001', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/New customer limit/select_SelectRECEIVEDPENDINGDEFERRED_1_2'), 
    '1000000001', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/New customer limit/select_SelectRECEIVEDPENDINGDEFERRED_1_2_3'), 
    '1000000001', true)

/*to upload image on below*/
WebUI.uploadFile(findTestObject('Object Repository/New customer limit/input_Last Updated Date_file'), file_kyc)

WebUI.uploadFile(findTestObject('Object Repository/New customer limit/input_SYSUSER_file'), file_kyc)

WebUI.uploadFile(findTestObject('Object Repository/New customer limit/input_SYSUSER_file_1'), file_kyc)

WebUI.uploadFile(findTestObject('Object Repository/New customer limit/input_SYSUSER_file_1_2'), file_kyc)

/*to upload image on below*/
WebUI.click(findTestObject('Object Repository/New customer limit/a_Save'))

WebUI.acceptAlert()

/* Documents starts here */
WebUI.click(findTestObject('Object Repository/New customer limit/a_NOTEPAD'))

WebUI.selectOptionByValue(findTestObject('Object Repository/New customer limit/select_SELECTDATA ENTRYAPPLICANT ENTRYVERIF_e6d432'), 
    '1000000001', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/New customer limit/select_SELECTNOTECVNOTEAUSER NOTECOLLN NOTE_7d6e22'), 
    '1000000001', true)

WebUI.setText(findTestObject('Object Repository/New customer limit/textarea_Description_description'), 'ok')

WebUI.click(findTestObject('Object Repository/New customer limit/a_Save_1'))

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/New customer limit/a_CR DECISION'))

WebUI.selectOptionByValue(findTestObject('Object Repository/New customer limit/select_SELECTHOLD REJECT RECOMMEND'), '1000000003', 
    true)

WebUI.setText(findTestObject('Object Repository/New customer limit/input_Remarks_remark'), 'OK')

WebUI.click(findTestObject('Object Repository/New customer limit/a_Save_1'))

WebUI.acceptAlert()

WebUI.waitForAlert(2)

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/New customer limit/img_Hi NAVINS_userr'))

WebUI.click(findTestObject('Object Repository/New customer limit/a_Logout'))

/* 2nd level recommendation - reenam */
WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/New customer limit/input_miFIN_userId'), 'reenam')

WebUI.setEncryptedText(findTestObject('Object Repository/New customer limit/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/New customer limit/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/New customer limit/i_CUSTOMER VIEWER_img1200006000'))

WebUI.click(findTestObject('Object Repository/New customer limit/i_LIMIT(LEASE)_img1200007001'))

WebUI.click(findTestObject('Object Repository/New customer limit/i_CUSTOMER LIMIT_img1200007003'))

WebUI.click(findTestObject('Object Repository/New customer limit/a_CR DECISION (1)'))

/* B1 - Eliminating search using limit code 

WebUI.setText(findTestObject('Page_miFIN/input_Limit Code_customerLimitCode'), 'LMCST0000006131')

 B1 ends here */
/* A2 - Changing logic, to search using first_name from excel */
WebUI.setText(findTestObject('Object Repository/New customer limit/input_Customer Name_customerName'), first_name)

/* A2 ends here*/
WebUI.click(findTestObject('Object Repository/New customer limit/input_To Date_search'))

WebUI.click(findTestObject('Object Repository/New customer limit/a_LMCST0000006131'))

WebUI.selectOptionByValue(findTestObject('Object Repository/New customer limit/select_SELECTHOLD REJECT RECOMMEND SEND BAC_346b4c'), 
    '1000000003', true)

WebUI.setText(findTestObject('Object Repository/New customer limit/input_Remarks_remark'), 'OK')

WebUI.click(findTestObject('Object Repository/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.waitForAlert(2)

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/New customer limit/img_Hi NAVINS_userr'))

WebUI.click(findTestObject('Object Repository/New customer limit/a_Logout'))

/* Lat level approval starts below - Poojab*/
WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/New customer limit/input_miFIN_userId'), 'poojab')

WebUI.setEncryptedText(findTestObject('Object Repository/New customer limit/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/New customer limit/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/New customer limit/i_CUSTOMER VIEWER_img1200006000'))

WebUI.click(findTestObject('Object Repository/New customer limit/i_LIMIT(LEASE)_img1200007001'))

WebUI.click(findTestObject('Object Repository/New customer limit/i_CUSTOMER LIMIT_img1200007003'))

WebUI.click(findTestObject('Object Repository/New customer limit/a_CR DECISION (1)'))

/*
WebUI.setText(findTestObject('Object Repository/New customer limit/input_Limit Code_customerLimitCode_1_2'), 'LMCST0000006131')
*/
/* A2 - Changing logic, to search using first_name from excel */
WebUI.setText(findTestObject('Object Repository/New customer limit/input_Customer Name_customerName'), first_name)

/* A2 ends here*/
WebUI.click(findTestObject('Object Repository/New customer limit/input_To Date_search'))

WebUI.click(findTestObject('Object Repository/New customer limit/a_LMCST0000006131'))

/* Guarantor - yes/no */
WebUI.click(findTestObject('Object Repository/New customer limit/a_LIMIT EDIT'))

WebUI.sendKeys(findTestObject('Object Repository/New customer limit/select_Guarantor'), 'yes')

WebUI.delay(5)

WebUI.click(findTestObject('Object Repository/New customer limit/a_Save_1'))

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/New customer limit/a_CR DECISION'))

WebUI.selectOptionByValue(findTestObject('Object Repository/New customer limit/select_SELECTHOLD REJECT APPROVE SEND BACK _4aa89b'), 
    '1000000004', true)

WebUI.setText(findTestObject('Object Repository/New customer limit/input_Remarks_remark'), 'OK')

WebUI.click(findTestObject('Object Repository/New customer limit/a_Save_1'))

WebUI.acceptAlert()

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/New customer limit/img_Hi NAVINS_userr'))

WebUI.click(findTestObject('Object Repository/New customer limit/a_Logout'))

WebUI.closeBrowser()

