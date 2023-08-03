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

WebUI.deleteAllCookies()

WebUI.navigateToUrl('https://mifinuat.cim.local/lease/')

WebUI.setText(findTestObject('Object Repository/all in one/input_miFIN_userId'), 'copsuserm')

WebUI.setEncryptedText(findTestObject('Object Repository/all in one/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/all in one/button_LOGIN'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.maximizeWindow()

WebUI.click(findTestObject('Object Repository/all in one/i_VIEWER_img1200004000'))

WebUI.click(findTestObject('Object Repository/all in one/i_DM QUOTATION_img1200004001'))

WebUI.click(findTestObject('Object Repository/all in one/a_DM APPLICATION'))

WebUI.click(findTestObject('Object Repository/all in one/input_Entity Code_btn btn-primary btn-sm'))

WebUI.waitForPageLoad(3)

WebUI.closeWindowTitle('miFIN')

WebUI.switchToWindowUrl('https://mifinuat.cim.local/lease/quotationSearchAction.do?actionPerformed=displaySearchScreen&searchType=DM&screenFlag=Y&parentBodyId=dmNewApplicantId')

/*

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTLEADCUSTOMERENQUIRYQUOTATION'), 'QUOTATION',
	true)

WebUI.setText(findTestObject('Object Repository/all in one/input_ENTITY CODE_entityCode'), 'QU00022832')

WebUI.setText(findTestObject('Object Repository/all in one/input_ENTITY CODE_entityCode_1_2'), 'QU00022832')

*/
WebUI.setText(findTestObject('Object Repository/all in one/input_CUSTOMERCOMPANY FNAME_firstName'), first_name)

WebUI.setText(findTestObject('Object Repository/all in one/input_LAST NAME_lastName'), last_name)

WebUI.click(findTestObject('Object Repository/all in one/input_Entity Code_btn btn-primary btn-sm'))

WebUI.click(findTestObject('Object Repository/all in one/input_MOBILE_selectApplicant'))

WebUI.click(findTestObject('Object Repository/all in one/input_ROHAN TESTQA_btn btn-primary btn-sm'))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/all in one/input_Entity Code_btn btn-primary btn-sm_1'))

WebUI.click(findTestObject('Object Repository/all in one/input_Limit Code_btndealerName'))

WebUI.switchToWindowTitle('limitChooser')

WebUI.sendKeys(findTestObject('Object Repository/all in one/input_Name_populateId'), Keys.chord(Keys.ENTER))

/*
WebUI.click(findTestObject('Object Repository/all in one/font_CNCIMF000003205 - ROHAN TESTQA , Finan_8dc407'))

WebUI.click(findTestObject('Object Repository/all in one/input_LMCST0000006150_OK'))

*/
WebUI.switchToWindowTitle('miFIN')

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTBUSINESS EXPANSIONEDUCATION LO_ad1c1d'), 
    '1000000021', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTAGN0000135-ALLY OOZEERALLYAGN0_388b55'), 
    '1000000135', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTCONTACT CENTREDEALERSHIPDIRECT_678662'), 
    '1200000033', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTALLY OOZEERALLYFABRICE DANGEOT_d85ecc'), 
    '1000000090', true)

WebUI.click(findTestObject('Object Repository/add guarantor/a_Save  Continue (1)'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

/* Saving Asset details */
WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

/* To input a guarantor */
WebUI.click(findTestObject('Object Repository/add guarantor/a_APPLICANT (1)'))

WebUI.click(findTestObject('Object Repository/add guarantor/input_APPLICANT LIST_newbutton (1)'))

WebUI.selectOptionByValue(findTestObject('Object Repository/add guarantor/select_SELECTCO-LESSEEGUARANTORBUYERBUYER LESSEE (1)'), 
    '1000000004', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/add guarantor/select_SELECTINDIVIDUALNON-INDIVIDUAL (1)'), '1000000001', 
    true)

WebUI.click(findTestObject('Object Repository/add guarantor/input_Customer Id_isExisting (1)'))

WebUI.click(findTestObject('Object Repository/add guarantor/input_Existing_search (1)'))

WebUI.switchToWindowUrl('https://mifinuat.cim.local/lease/applicantSearchAction.do?actionPerformed=displaySearchScreen&productId=&limitId=&limitAppFlag=DM')

WebUI.setText(findTestObject('Object Repository/add guarantor/input_COMPANY NAME FIRST NAME_firstName_1_2_3_4_5_6_7_8 (1)'), 
    'jonathan')

WebUI.setText(findTestObject('Object Repository/add guarantor/input_LAST NAME_lastName (1)'), 'TESTQA')

WebUI.click(findTestObject('Object Repository/add guarantor/input_DOB INCORP_blueBotton (1)'))

WebUI.click(findTestObject('Object Repository/add guarantor/input_MOBILE NO_selectApplicant (1)'))

WebUI.click(findTestObject('Object Repository/add guarantor/input_JONATHAN  TESTQA_blueBotton (1)'))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/Page_miFIN/input_Existing_get'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTSPOUSEFATHERMOTHERSONDAUGHTERS_92032f'), 
    '1000000011', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECT  AFGHANISTANALBANIAALGERIAAME_60d157'), 
    '1000000103', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECT  AFGHANISTANALBANIAALGERIAAME_60d157_1'), 
    '1000000103', true)

WebUI.click(findTestObject('Object Repository/Page_miFIN/i_Passport Expiry_fa fa-calendar'))

WebUI.switchToWindowTitle('Calendar')

WebUI.click(findTestObject('Object Repository/Page_Calendar/a_8'))

WebUI.switchToWindowTitle('miFIN')

WebUI.waitForPageLoad(3)

WebUI.click(findTestObject('Object Repository/Page_miFIN/a_Save (1)'))

WebUI.waitForAlert(3)

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/add guarantor/a_APPLICANT (1)'))

WebUI.takeScreenshot()

/* Input guarantor ends here */
WebUI.click(findTestObject('Object Repository/additional items/a_ASSET DETAILS'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/all in one/a_QUOTATION_1'))

WebUI.click(findTestObject('Object Repository/all in one/select_SELECTMONTHLY'))

WebUI.sendKeys(findTestObject('Object Repository/all in one/select_SELECTMONTHLY'), 'Monthly')

WebUI.click(findTestObject('Object Repository/all in one/select_SELECT17142128'))

WebUI.sendKeys(findTestObject('Object Repository/all in one/select_SELECT17142128'), '7', FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/all in one/input_Customer Bank_btnBank'))

WebUI.switchToWindowTitle('CUSTOMER BANK')

WebUI.click(findTestObject('Object Repository/all in one/font_CIM BANK - PORT LOUIS - CURRENT ACCOUN_6a03f4'))

WebUI.click(findTestObject('Object Repository/all in one/input_Name_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/all in one/input_CIM Bank_btnBank2'))

WebUI.switchToWindowTitle('CIM BENEFECIARY')

WebUI.click(findTestObject('Object Repository/all in one/font_ABC BANKING CORPORATION LTD - HEAD OFF_3e22ab'))

WebUI.click(findTestObject('Object Repository/all in one/input_STANDARD BANK (MAURITIUS) LTD - HEAD _d26a3e'))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

WebUI.waitForPageLoad(5)

/* CashFlow */
WebUI.click(findTestObject('Object Repository/all in one/a_CASHFLOW_1'))

WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

/* DM Quotation > registration details */
WebUI.click(findTestObject('Object Repository/all in one/a_REGISTRATION DETAILS_1'))

WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

/* to change here 

WebUI.click(findTestObject('Object Repository/all in one/a_DM SANCTION'))

*/
WebUI.click(findTestObject('Object Repository/all in one/a_DOCUMENT'))

WebUI.click(findTestObject('Object Repository/all in one/input_Last Updated Date_UPDATED_CHK0'))

WebUI.click(findTestObject('Object Repository/all in one/input_SYSUSER_UPDATED_CHK1'))

WebUI.setText(findTestObject('Object Repository/all in one/input_Last Updated Date_DOCUMENT_DESCRIPTION0'), 'ok')

WebUI.setText(findTestObject('Object Repository/all in one/input_SYSUSER_DOCUMENT_DESCRIPTION1'), 'ok')

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SelectRECEIVEDPENDINGDEFERRED'), '1000000001', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SelectRECEIVEDPENDINGDEFERRED_1'), '1000000001', 
    true)

WebUI.uploadFile(findTestObject('Object Repository/additional items/input_Last Updated Date_file'), file_kyc)

WebUI.uploadFile(findTestObject('Object Repository/additional items/input_SYSUSER_file'), file_kyc)

WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/all in one/a_DM OFFLINE ACTION (1)'))

WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

WebUI.waitForPageLoad(5)

/* DM SANCTION STARTS BELOW */
WebUI.click(findTestObject('Object Repository/all in one/a_DM SANCTION'))

WebUI.setText(findTestObject('Object Repository/all in one/input_Engine No_engineNo'), '832102')

WebUI.setText(findTestObject('Object Repository/all in one/input_Chassis No_chassisNo'), '06382')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTCLIENTFMC (1)'), '1000000001', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTHOLD REJECT RECOMMEND'), '1000000003', 
    true)

WebUI.setText(findTestObject('Object Repository/all in one/input_Remarks_remark'), 'ok')

WebUI.click(findTestObject('Object Repository/all in one/a_Save_1'))

WebUI.waitForAlert(2, FailureHandling.OPTIONAL)

WebUI.acceptAlert()

WebUI.waitForAlert(2, FailureHandling.OPTIONAL)

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/all in one/img_Hi COPSUSERM_userr (1)'))

WebUI.click(findTestObject('Object Repository/all in one/a_Logout (1)'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/all in one/input_miFIN_userId'), 'farzanan')

WebUI.setEncryptedText(findTestObject('Object Repository/all in one/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/all in one/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/all in one/i_VIEWER_img1200004000 (1)'))

WebUI.click(findTestObject('Object Repository/all in one/i_DM APPLICATION_img1200004003'))

WebUI.click(findTestObject('Object Repository/all in one/a_DM SANCTION (1)'))

/* To search by first_name and last_name */
/* Approving DM Quote */
/*

WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_dmCode'), 'DMFIN1000008259')

WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_dmCode_1'), 'DMFIN1000008259')

*/
WebUI.setText(findTestObject('Object Repository/additional items/input_Customer Name_firstName'), first_name)

WebUI.setText(findTestObject('Object Repository/additional items/input_National IDBRNPP No_generalFilter'), nic_num)

WebUI.click(findTestObject('Object Repository/all in one/input_Delivery Date_search'))

WebUI.click(findTestObject('Object Repository/all in one/a_DMFIN1000008259'))

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTHOLD REJECT APPROVE SEND BACK _efe445'), 
    '1000000004', true)

WebUI.setText(findTestObject('Object Repository/all in one/input_Remarks_remark'), 'ok')

WebUI.click(findTestObject('Object Repository/all in one/a_Save_1'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/all in one/div_STANDARD_tgl_nbtn'))

WebUI.click(findTestObject('Object Repository/all in one/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Object Repository/all in one/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/all in one/input_miFIN_userId'), 'copsuserm')

WebUI.setEncryptedText(findTestObject('Object Repository/all in one/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/all in one/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

/*------------------------------- Dm Offline action starts here--------------------------------------------------- */
WebUI.click(findTestObject('Object Repository/all in one/i_VIEWER_img1200004000 (1)'))

WebUI.click(findTestObject('Object Repository/all in one/i_DM APPLICATION_img1200004003'))

WebUI.click(findTestObject('Object Repository/all in one/a_DM OFFLINE ACTION'))

/*
WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_dmCode'), 'DMFIN1000008259')

WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_dmCode_1'), 'DMFIN1000008259')

*/
WebUI.setText(findTestObject('Object Repository/additional items/input_Customer Name_firstName'), first_name)

WebUI.setText(findTestObject('Object Repository/additional items/input_National IDBRNPP No_generalFilter'), nic_num)

WebUI.click(findTestObject('Object Repository/all in one/input_Delivery Date_search'))

WebUI.click(findTestObject('Object Repository/all in one/a_DMFIN1000008259'))

WebUI.setText(findTestObject('Object Repository/all in one/input_Expected Delivery Date_EXP_DELIVERY_DT'), '10-jan-2024')

WebUI.setText(findTestObject('Object Repository/all in one/input_Actual Delivery Date_ACT_DELIVERY_DT'), '10-jan-2024')

WebUI.setText(findTestObject('Object Repository/all in one/input_REMARKS_REMARKS'), 'ok')

WebUI.setText(findTestObject('Object Repository/all in one/input_Original Reg. No_TEMPORARY_REG_NO'), '2345678')

WebUI.click(findTestObject('Object Repository/all in one/input_Sales Deed Released_ng-pristine ng-un_bb831b'))

WebUI.setText(findTestObject('Object Repository/all in one/input_Registration Date_REG_DT'), '10-jan-2024')

WebUI.click(findTestObject('Object Repository/all in one/input_Assignation in favour of CFSL_INS_STA_d425d5'))

WebUI.setText(findTestObject('Page_miFIN/input_Assignation in favour of CFSL_INS_END_DATE0'), '10-JAN-2022')

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECT EAGLE INSURANCE LIMITEDPHOENI_523736'), 
    '1000000001', true)

WebUI.setText(findTestObject('Object Repository/all in one/input_Assignation in favour of CFSL_POLICY_NO0'), '345678')

WebUI.doubleClick(findTestObject('Object Repository/all in one/input_Assignation in favour of CFSL_SUM_INSURED0'))

WebUI.setText(findTestObject('Object Repository/all in one/input_Assignation in favour of CFSL_SUM_INSURED0'), '100')

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECT YES NO'), 'Y', true)

WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

/* Dm Offline action ends here */
/*-------------------------------------------------------------------- MAKER--------------------------------------------------------- */
WebUI.click(findTestObject('Object Repository/all in one/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Object Repository/all in one/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/all in one/input_miFIN_userId'), 'navind')

WebUI.setEncryptedText(findTestObject('Object Repository/all in one/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/all in one/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/all in one/i_DM VIEWER_img1000000017'))

WebUI.click(findTestObject('Object Repository/all in one/i_MAKER_img1000000034'))

WebUI.click(findTestObject('Object Repository/all in one/a_MAKER'))

/*
WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_prospectNo'), 'DMFIN1000008259')

WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_prospectNo_1'), 'DMFIN1000008259')

*/
WebUI.setText(findTestObject('Object Repository/additional items/input_Customer Name_firstName'), first_name)

WebUI.setText(findTestObject('Object Repository/additional items/input_National IDBRNPP No_generalFilter'), nic_num)

WebUI.click(findTestObject('Object Repository/all in one/input_Revaluation Pending_search'))

WebUI.click(findTestObject('Object Repository/all in one/a_DMFIN1000008259_1'))

WebUI.setText(findTestObject('Object Repository/all in one/input__txtf_rcReceiptAmount_1_2_3'), '300')

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTHEAD OFFICE_1'), '1000000037', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECT     CUSTOMERTHIRD PARTY'), '1000000001', 
    true)

WebUI.click(findTestObject('Object Repository/all in one/div_Entity Type  SELECT     CUSTOMERTHIRD P_137bcf'))

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECT     CASHBANK TRANSFERCHEQUECR_23534d'), 
    '1000000002', true)

WebUI.click(findTestObject('Object Repository/all in one/input__btnDepositeBankName'))

WebUI.switchToWindowTitle('Bank')

WebUI.click(findTestObject('Object Repository/all in one/font_CIM BANK'))

WebUI.click(findTestObject('Object Repository/all in one/input_STANDARD BANK (MAURITIUS) LTD_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/all in one/input__btnDepositeBranchName'))

WebUI.switchToWindowTitle('Branch')

WebUI.click(findTestObject('Object Repository/all in one/font_PORT LOUIS'))

WebUI.click(findTestObject('Object Repository/all in one/input_PORT LOUIS_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/all in one/input__btnAutoAccountName'))

WebUI.switchToWindowTitle('Account')

WebUI.click(findTestObject('Object Repository/all in one/font_123456789'))

WebUI.click(findTestObject('Object Repository/all in one/input_Name_OK (1)'))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/all in one/input_Send To Author_mrSendToAuthor'))

WebUI.setText(findTestObject('Object Repository/all in one/textarea_Remarks_mrRemarks'), 'o')

WebUI.setText(findTestObject('Object Repository/all in one/textarea_Remarks_mrRemarks_1'), 'ok')

WebUI.click(findTestObject('Object Repository/all in one/a_Save_1_2'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/all in one/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Object Repository/all in one/a_Logout'))

/*------------------------------------------------------Author-------------------------------------------------------------*/
WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/all in one/input_miFIN_userId'), 'farzanan')

WebUI.setEncryptedText(findTestObject('Object Repository/all in one/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/all in one/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/all in one/i_DM VIEWER_img1000000017'))

WebUI.click(findTestObject('Object Repository/all in one/i_MAKER_img1000000034'))

WebUI.click(findTestObject('Object Repository/all in one/a_AUTHOR'))

WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_prospectNo'), 'DMFIN1000008259')

WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_prospectNo_1'), 'DMFIN1000008259')

WebUI.click(findTestObject('Object Repository/all in one/input_Revaluation Pending_search'))

WebUI.click(findTestObject('Object Repository/all in one/a_DMFIN1000008259_1_2'))

WebUI.click(findTestObject('Object Repository/all in one/input_Approve_arApprove'))

WebUI.setText(findTestObject('Object Repository/all in one/textarea_Remarks_arRemarks'), 'ok')

WebUI.click(findTestObject('Object Repository/all in one/a_Save_1_2'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/all in one/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Object Repository/all in one/a_Logout'))

/*---------------------------------------------------DM Offline Action----------------------------------------------*/
WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/all in one/input_miFIN_userId'), 'copsuserm')

WebUI.setEncryptedText(findTestObject('Object Repository/all in one/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/all in one/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/all in one/i_VIEWER_img1200004000 (1)'))

WebUI.click(findTestObject('Object Repository/all in one/i_DM APPLICATION_img1200004003'))

WebUI.click(findTestObject('Object Repository/all in one/a_DM OFFLINE ACTION'))

/*
WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_dmCode'), 'DMFIN1000008259')

WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_dmCode_1'), 'DMFIN1000008259')
*/
WebUI.setText(findTestObject('Object Repository/additional items/input_Customer Name_firstName'), first_name)

WebUI.setText(findTestObject('Object Repository/additional items/input_National IDBRNPP No_generalFilter'), nic_num)

WebUI.click(findTestObject('Object Repository/all in one/input_Delivery Date_search'))

WebUI.click(findTestObject('Object Repository/all in one/a_DMFIN1000008259'))

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTABC MOTORS COMPANY LIMITED'), '1000027212', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTCHEQUERTGSBANK TRANSFERCASHDEM_da6496'), 
    '1000000002', true)

WebUI.click(findTestObject('Object Repository/all in one/input_ADDITIONAL CHARGE DETAILS_btn btn-pri_82522f'))

WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/all in one/a_DOCUMENT'))

WebUI.click(findTestObject('Object Repository/all in one/a_Post Sanction'))

WebUI.click(findTestObject('Object Repository/all in one/input_Last Updated Date_UPDATED_CHK0'))

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SelectRECEIVEDPENDINGDEFERRED'), '1000000001', 
    true)

WebUI.setText(findTestObject('Object Repository/all in one/input_Last Updated Date_DOCUMENT_DESCRIPTION0'), 'ok')

WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

/*------------------------------------Approve Capitalisation------------------------------------------------*/
WebUI.setText(findTestObject('Object Repository/all in one/input_miFIN_userId'), 'farzanan')

WebUI.setEncryptedText(findTestObject('Object Repository/all in one/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/all in one/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/all in one/div_DM QUOTATION'))

WebUI.click(findTestObject('Object Repository/all in one/i_VIEWER_img1200004000 (1)'))

WebUI.click(findTestObject('Object Repository/all in one/i_DM APPLICATION_img1200004003'))

WebUI.click(findTestObject('Object Repository/all in one/a_CAPITALISATION  AUTHOR'))

/*

WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_dmCode'), 'DMFIN1000008259')

WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_dmCode_1'), 'DMFIN1000008259')

*/
WebUI.setText(findTestObject('Object Repository/additional items/input_Customer Name_firstName'), first_name)

WebUI.setText(findTestObject('Object Repository/additional items/input_National IDBRNPP No_generalFilter'), nic_num)

WebUI.click(findTestObject('Object Repository/all in one/input_Revaluation Pending_search'))

WebUI.click(findTestObject('Object Repository/all in one/a_DMFIN1000008259'))

WebUI.click(findTestObject('Object Repository/all in one/a_Save_1'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/all in one/span'))

WebUI.click(findTestObject('Object Repository/all in one/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Object Repository/all in one/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/all in one/input_miFIN_userId'), 'copsuserm')

WebUI.setEncryptedText(findTestObject('Object Repository/all in one/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/all in one/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/all in one/i_DM VIEWER_img1000000017'))

WebUI.click(findTestObject('Object Repository/all in one/i_INVOICE_img1200106858'))

WebUI.click(findTestObject('Object Repository/all in one/a_MAKER_1'))

/* to change below, to search by name*/
WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_prospectNo'), 'DMFIN1000008259')

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

WebUI.click(findTestObject('Object Repository/all in one/input_SYSUSER_UPDATED_CHK6'))

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

WebUI.setText(findTestObject('Object Repository/all in one/input_SYSUSER_DOCUMENT_DESCRIPTION6'), 'OK')

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
    '1000000001', true)

WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

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

WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_prospectNo_1'), 'DMFIN1000008259')

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

WebUI.comment('Payment maker checker starts here')

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/all in one/input_miFIN_userId'), 'copsuserm')

WebUI.setEncryptedText(findTestObject('Object Repository/all in one/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/all in one/button_LOGIN'))

WebUI.click(findTestObject('Object Repository/all in one/div_LEASE RESIDUAL VALUE UPLOAD_toggle-button'))

WebUI.click(findTestObject('Object Repository/all in one/i_DM APPLICATION_img1200004003'))

WebUI.click(findTestObject('Object Repository/all in one/a_DM OFFLINE ACTION'))

/*

WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_dmCode'), 'DMFIN1000008259')

WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_dmCode_1'), 'DMFIN1000008259')

*/
WebUI.setText(findTestObject('Object Repository/additional items/input_Customer Name_firstName'), first_name)

WebUI.setText(findTestObject('Object Repository/additional items/input_National IDBRNPP No_generalFilter'), nic_num)

WebUI.click(findTestObject('Object Repository/all in one/input_Delivery Date_search'))

WebUI.click(findTestObject('Object Repository/all in one/a_DMFIN1000008259'))

WebUI.click(findTestObject('Object Repository/all in one/span'))

WebUI.click(findTestObject('Object Repository/all in one/div_PROSPECT NODMFIN1000008259             _01f497'))

WebUI.click(findTestObject('Object Repository/all in one/div_STANDARD_tgl_nbtn tglingSpan'))

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTABC MOTORS COMPANY LIMITED'), '1000027212', 
    true)

WebUI.click(findTestObject('Object Repository/all in one/input_OperationsUser_ROW_SELECT0'))

WebUI.click(findTestObject('Object Repository/all in one/input_SPA_ROW_SELECT1'))

WebUI.click(findTestObject('Object Repository/all in one/input_SPA_ROW_SELECT2'))

WebUI.click(findTestObject('Object Repository/all in one/input_SPA_SCHEDULE_PAYMENT0'))

WebUI.click(findTestObject('Object Repository/all in one/input_SPA_SCHEDULE_PAYMENT1'))

WebUI.click(findTestObject('Object Repository/all in one/input_SPA_SCHEDULE_PAYMENT2'))

WebUI.click(findTestObject('Object Repository/all in one/input_SPA_PAYMENT_DATE0'))

WebUI.click(findTestObject('Object Repository/all in one/tr_SPA                        SELECTFIRST R_910086'))

WebUI.setText(findTestObject('Object Repository/all in one/input_SPA_PAYMENT_DATE0_1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16_17_18_19_20_21_22_23_24_25_26_27_28'), 
    '13-JAN-2023')

WebUI.click(findTestObject('Object Repository/all in one/tr_SPA                        SELECTFIRST R_910086'))

WebUI.setText(findTestObject('Object Repository/all in one/input_SPA_PAYMENT_DATE0_1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16_17_18_19_20_21_22_23_24_25_26_27_28'), 
    '13-JAN-2023')

WebUI.click(findTestObject('Object Repository/all in one/tr_SPA                        SELECTFIRST R_910086'))

WebUI.setText(findTestObject('Object Repository/all in one/input_SPA_PAYMENT_DATE1_1_2_3'), '13-JAN-2023')

WebUI.click(findTestObject('Object Repository/all in one/tr_SPA                        SELECTFIRST R_910086'))

WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

WebUI.click(findTestObject('Object Repository/all in one/div_LEASE RESIDUAL VALUE UPLOAD_toggle-button'))

WebUI.click(findTestObject('Object Repository/all in one/i_VIEWER_img1200004029'))

WebUI.click(findTestObject('Object Repository/all in one/a_MAKER'))

WebUI.setText(findTestObject('Object Repository/all in one/input_Prospect Code_colProspectCode'), 'DMFIN1000008259')

WebUI.setText(findTestObject('Object Repository/all in one/input_Prospect Code_colProspectCode_1'), 'DMFIN1000008259')

WebUI.click(findTestObject('Object Repository/all in one/div_Prospect Code                          _efcb6b'))

WebUI.setText(findTestObject('Object Repository/all in one/input_Payment To_paymentTo'), '')

WebUI.click(findTestObject('Object Repository/all in one/div_Prospect Code                          _efcb6b'))

WebUI.setText(findTestObject('Object Repository/all in one/input_Payment From_paymentFrom'), '')

WebUI.click(findTestObject('Object Repository/all in one/input_Payment To_searchBtn'))

WebUI.click(findTestObject('Object Repository/all in one/input_Schedule Payment (Insurance)_ng-prist_50993b'))

WebUI.click(findTestObject('Object Repository/all in one/input_Send To Author_mrSendToAuthor'))

WebUI.setText(findTestObject('Object Repository/all in one/input_DMFIN1000008259_instNo'), '57387943')

WebUI.setText(findTestObject('Object Repository/all in one/input_DMFIN1000008259_instDate'), '20-NOV-2022')

WebUI.setText(findTestObject('Object Repository/all in one/input_DMFIN1000008259_mRemarks'), 'OK')

WebUI.click(findTestObject('Object Repository/all in one/a_Save_1'))

WebUI.click(findTestObject('Object Repository/all in one/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Object Repository/all in one/a_Logout'))

WebUI.switchToWindowTitle('')

WebUI.setText(findTestObject('Object Repository/all in one/input_miFIN_userId'), 'FARZANAN')

WebUI.setEncryptedText(findTestObject('Object Repository/all in one/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/all in one/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/all in one/div_DM QUOTATION'))

WebUI.click(findTestObject('Object Repository/all in one/i_VIEWER_img1200004000'))

WebUI.click(findTestObject('Object Repository/all in one/i_VIEWER_img1200004029 (1)'))

WebUI.click(findTestObject('Object Repository/all in one/a_AUTHOR'))

WebUI.click(findTestObject('Object Repository/all in one/input_Schedule Payment (Insurance)_ng-prist_50993b'))

WebUI.click(findTestObject('Object Repository/all in one/input_Approve_arApprove'))

WebUI.setText(findTestObject('Object Repository/all in one/input_DMFIN1000008259_aRemarks'), 'o')

WebUI.setText(findTestObject('Object Repository/all in one/input_DMFIN1000008259_aRemarks_1'), 'ook')

WebUI.setText(findTestObject('Object Repository/all in one/input_DMFIN1000008259_aRemarks_1_2'), 'ok')

WebUI.setText(findTestObject('Object Repository/all in one/input_DMFIN1000008259_aRemarks_1_2_3'), 'ok')

WebUI.click(findTestObject('Object Repository/all in one/a_Save_1'))

WebUI.click(findTestObject('Object Repository/all in one/div_LEASE RESIDUAL VALUE UPLOAD_toggle-button'))

WebUI.click(findTestObject('Object Repository/all in one/a_DM VIEWER'))

/*

WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_dmCode'), 'DMFIN1000008259')

WebUI.setText(findTestObject('Object Repository/all in one/input_DM Code_dmCode_1'), 'DMFIN1000008259')

*/
WebUI.setText(findTestObject('Object Repository/additional items/input_Customer Name_firstName'), first_name)

WebUI.setText(findTestObject('Object Repository/additional items/input_National IDBRNPP No_generalFilter'), nic_num)

WebUI.click(findTestObject('Object Repository/all in one/input_Delivery Date_search'))

WebUI.click(findTestObject('Object Repository/all in one/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Object Repository/all in one/a_Logout'))

WebUI.switchToWindowTitle('')

WebUI.setText(findTestObject('Object Repository/all in one/input_miFIN_userId'), 'copsuserm')

WebUI.setEncryptedText(findTestObject('Object Repository/all in one/input_miFIN_password'), 'VuZ2hn51ueg=')

WebUI.comment('Run EOD > same required to proceed to foreclosure')

WebUI.openBrowser('')

WebUI.navigateToUrl('https://mifinuat.cim.local/lease/userAuthAction.do;jsessionid=Gj-oZPbAF3Mhm4X3W5A16o2CilLcX6l9Q_RG25VL.ho-mfinlos-uat?dispatchMethod=userAuth')

WebUI.click(findTestObject('Object Repository/Page_miFIN/input__btnBranch'))

WebUI.switchToWindowTitle('Branch/Cashpoint')

WebUI.click(findTestObject('Object Repository/Page_BranchCashpoint/font_CIM ROSE HILL'))

WebUI.click(findTestObject('Object Repository/Page_BranchCashpoint/input_Y CADER CO LTD PORT LOUIS_OK'))

