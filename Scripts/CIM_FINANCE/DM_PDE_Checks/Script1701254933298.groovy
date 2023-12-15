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

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_miFIN_userId'), 'copsuserm')

WebUI.setEncryptedText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/button_LOGIN'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.maximizeWindow()

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/i_VIEWER_img1200004000'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_DM VIEWER'))

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_National IDBRNPP No_generalFilter'), nic_num)

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Engine No_search'))

String dmCode = WebUI.getText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_DMFIN1000008514'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/i_DM VIEWER_img1000000017'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/i_INVOICE_img1200106858'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_MAKER'))

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_DM Code_prospectNo'), dmCode)

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Engine No_search'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_DMFIN1000008259'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_EMPLOYMENT  FINANCIAL INFO'))

WebUI.selectOptionByValue(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/select_SELECT                         CURRE_28079a'), 
    '1000000002', true)

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_MICR Code_btnEditBankName'))

WebUI.switchToWindowTitle('Bank')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_Bank/font_HONG KONG AND SHANGHAI BANK LTD'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_Bank/input_STANDARD BANK (MAURITIUS) LTD_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_MICR Code_btnEditBranchName'))

WebUI.switchToWindowTitle('Branch')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_Branch/font_PORT LOUIS'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_Branch/input_VACOAS_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Page_miFIN/input_MICR Code_accountNameId'), '123124')

WebUI.selectOptionByValue(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/select_SELECTUSDGBPEURMUR'), '1000000001', 
    true)

WebUI.scrollToElement(findTestObject('DMPDE_Checks/Page_miFIN/div_OTHER LOANS DETAILS'), 0)

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Prof. Inc.  Basic Sal.  Declared Inc__4e9d9c'), 
    FailureHandling.STOP_ON_FAILURE)

WebUI.doubleClick(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Prof. Inc.  Basic Sal.  Declared Inc__4e9d9c'), 
    FailureHandling.STOP_ON_FAILURE)

WebUI.sendKeys(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Prof. Inc.  Basic Sal.  Declared Inc__4e9d9c'), 
    '10000')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Other Income_otherIncomeDetail'), FailureHandling.STOP_ON_FAILURE)

WebUI.doubleClick(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Other Income_otherIncomeDetail'), FailureHandling.STOP_ON_FAILURE)

WebUI.sendKeys(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Other Income_otherIncomeDetail'), '500')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_APPLICANT'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Y_sendToAuthor'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_My Dashboard'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_DASHBOARD/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_DASHBOARD/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_miFIN_userId'), 'farzanan')

WebUI.setEncryptedText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_DASHBOARD/i_DM VIEWER_img1000000017'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_DASHBOARD/i_INVOICE_img1200106858'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_DASHBOARD/a_AUTHOR'))

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_DM Code_prospectNo'), dmCode)

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Engine No_search'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_DMFIN1000008259'))

WebUI.waitForElementClickable(findTestObject('DMPDE_Checks/Page_miFIN/a_LESSEE'), 0, FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_LESSEE'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Approve_arApprove'))

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/textarea_Remarks_arRemarks'), 'ok')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_Save_1'))

WebUI.acceptAlert()

WebUI.click(findTestObject('DMPDE_Checks/Page_miFIN/img_Hi FARZANAN_userr_1'))

WebUI.click(findTestObject('DMPDE_Checks/Page_miFIN/a_Logout_1(0)'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_miFIN_userId'), 'copsuserm')

WebUI.setEncryptedText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_DASHBOARD/i_DM VIEWER_img1000000017'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_DASHBOARD/i_INVOICE_img1200106858'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_DASHBOARD/a_MAKER'))

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_DM Code_prospectNo'), dmCode)

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Engine No_search'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_DMFIN1000008259'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_FORECLOSURE'))

WebUI.click(findTestObject('DMPDE_Checks/Page_miFIN/input_Waiver of FC Charge_waiverAllowed'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_Save_1_2'))

WebUI.delay(3, FailureHandling.OPTIONAL)

WebUI.acceptAlert()

WebUI.doubleClick(findTestObject('DMPDE_Checks/DM_PDE/Page_miFIN/input_penalty_TENURE_TO0'))

WebUI.setText(findTestObject('DMPDE_Checks/DM_PDE/Page_miFIN/input_penalty_TENURE_TO0'), '')

WebUI.setText(findTestObject('DMPDE_Checks/DM_PDE/Page_miFIN/input_penalty_TENURE_TO0'), '3')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_Save_1_2'))

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('DMPDE_Checks/Page_miFIN/img_Hi COPSUSERM_userr_1'))

WebUI.click(findTestObject('DMPDE_Checks/Page_miFIN/a_Logout_1(C)'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_miFIN_userId'), 'farzanan')

WebUI.setEncryptedText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_DASHBOARD/i_DM VIEWER_img1000000017'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_DASHBOARD/i_INVOICE_img1200106858'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_DASHBOARD/a_AUTHOR'))

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_DM Code_prospectNo'), dmCode)

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Engine No_search'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_DMFIN1000008259'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_FORECLOSURE'))

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Remarks_AUTHOR_REMARKS_ID'), 'ok')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_Save_1_2'))

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('DMPDE_Checks/Page_miFIN/img_Hi FARZANAN_userr_1'))

WebUI.click(findTestObject('DMPDE_Checks/Page_miFIN/a_Logout_1(0)'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_miFIN_userId'), 'copsuserm')

WebUI.setEncryptedText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_DASHBOARD/i_DM VIEWER_img1000000017'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_DASHBOARD/i_INVOICE_img1200106858'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_DASHBOARD/a_MAKER'))

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_DM Code_prospectNo'), dmCode)

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Engine No_search'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_DMFIN1000008259'))

WebUI.click(findTestObject('DMPDE_Checks/Page_miFIN/a_REGISTRATION AND ASSET DETAILS'))

WebUI.doubleClick(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Engine No_ENGINE_NO'))

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Engine No_ENGINE_NO'), '9999')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Chassis No_CHASSIS_NO'))

WebUI.doubleClick(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Chassis No_CHASSIS_NO'))

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Chassis No_CHASSIS_NO'), '999')

WebUI.scrollToElement(findTestObject('DMPDE_Checks/Page_miFIN/div_Customer Bank(C)'), 0)

WebUI.click(findTestObject('Object Repository/Repositry/Page_miFIN/input_INSURANCE_btn btn-primary btn-sm'))

WebUI.setText(findTestObject('Repositry/Page_miFIN/input_Assignation in favour of CFSL_INS_STA_05ce27'), '11-JAN-2024')

WebUI.setText(findTestObject('Repositry/Page_miFIN/input_Assignation in favour of CFSL_INS_END_DATE1'), '12-JAN-2025')

WebUI.selectOptionByValue(findTestObject('Repositry/Page_miFIN/select_EAGLE INSURANCE LIMITEDPHOENIX INSUR_f5c385'), '1000000002', 
    true)

WebUI.setText(findTestObject('Repositry/Page_miFIN/input_Assignation in favour of CFSL_POLICY_NO1'), '1920')

WebUI.click(findTestObject('Repositry/Page_miFIN/input_Assignation in favour of CFSL_SUM_INSURED1'))

WebUI.doubleClick(findTestObject('Repositry/Page_miFIN/input_Assignation in favour of CFSL_SUM_INSURED1'))

WebUI.setText(findTestObject('Repositry/Page_miFIN/input_Assignation in favour of CFSL_SUM_INSURED1'), '150')

WebUI.selectOptionByValue(findTestObject('Repositry/Page_miFIN/select_SELECT YES NO'), 'Y', true)

WebUI.scrollToElement(findTestObject('DMPDE_Checks/Page_miFIN/div_Remarks_(C)'), 0)

WebUI.click(findTestObject('DM_Offline_Ins/Page_miFIN/Page_miFIN/Page_miFIN/Page_miFIN/a_Save_(RA1)'))

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('DMPDE_Checks/Page_miFIN/img_Hi COPSUSERM_userr(RA)'))

WebUI.click(findTestObject('DMPDE_Checks/Page_miFIN/a_Logout(RA)'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_miFIN_userId'), 'farzanan')

WebUI.setEncryptedText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_DASHBOARD/i_DM VIEWER_img1000000017'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_DASHBOARD/i_INVOICE_img1200106858'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_DASHBOARD/a_AUTHOR'))

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_DM Code_prospectNo'), dmCode)

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Engine No_search'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_DMFIN1000008259'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_REGISTRATION AND ASSET DETAILS'))

WebUI.scrollToElement(findTestObject('DMPDE_Checks/Page_miFIN/div_Remarks'), 0)

WebUI.scrollToElement(findTestObject('DMPDE_Checks/Page_miFIN/div_Mobile No'), 0)

WebUI.scrollToElement(findTestObject('DMPDE_Checks/Page_miFIN/div_AUTHORS REMARKS'), 0)

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Approval Decision ApproveReject_appDecision'))

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Remarks_AUTHOR_REMARKS_ID'), 'ok')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_Save_1_2'))

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

String customerCode = WebUI.getText(findTestObject('Object Repository/Repositry/Page_miFIN/div_CNCIMF000003205'))

WebUI.click(findTestObject('DMPDE_Checks/Page_miFIN/img_Hi FARZANAN_userr_1'))

WebUI.click(findTestObject('DMPDE_Checks/Page_miFIN/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_miFIN_userId'), 'copsuserm')

WebUI.setEncryptedText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/i_CHANGE PASSWORD_img1000002041'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/a_CUSTOMER VIEWER'))

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Applicant Code_customerCode'), customerCode)

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/Page_miFIN/input_Engine No_search'))

WebUI.click(findTestObject('DMPDE_Checks/Page_miFIN/a_CNCIMF000003808'))

WebUI.click(findTestObject('DMPDE_Checks/Page_miFIN/a_FINANCIAL  OTHER INFO_(C)'))

WebUI.delay(5, FailureHandling.OPTIONAL)

WebUI.scrollToElement(findTestObject('DMPDE_Checks/Page_miFIN/div_OTHER LOANS DETAILS_1'), 0)

WebUI.scrollToElement(findTestObject('DMPDE_Checks/Page_miFIN/td_Account Type'), 0)

WebUI.click(findTestObject('DMPDE_Checks/Page_miFIN/div_LEASE RESIDUAL VALUE UPLOAD_toggle-button_(C)'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/DM_PDE/Page_miFIN/i_VIEWER_img1200004000'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/DM_PDE/Page_miFIN/i_DM APPLICATION_img1200004003'))

WebUI.scrollToElement(findTestObject('DMPDE_Checks/DM_PDE/Page_miFIN/a_PENDING ACTIVITIES'), 0)

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/DM_PDE/Page_miFIN/a_DM OFFLINE ACTION'))

WebUI.setText(findTestObject('Object Repository/DMPDE_Checks/DM_PDE/Page_miFIN/input_DM Code_dmCode'), dmCode)

WebUI.click(findTestObject('DMPDE_Checks/DM_PDE/Page_miFIN/input_Engine No_search'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/DM_PDE/Page_miFIN/a_DMFIN1000008587'))

WebUI.scrollToElement(findTestObject('DMPDE_Checks/DM_PDE/Page_miFIN/div_Dry Capitalisation'), 0)

WebUI.delay(5, FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/DM_PDE/Page_miFIN/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Object Repository/DMPDE_Checks/DM_PDE/Page_miFIN/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

