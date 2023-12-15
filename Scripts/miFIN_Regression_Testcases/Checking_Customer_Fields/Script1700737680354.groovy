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

WebUI.setText(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/input_miFIN_userId'), 'copsuserm')

WebUI.setEncryptedText(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/Checking_Customer_Fields/Page_DASHBOARD/i_CHANGE PASSWORD_img1000002041'))

WebUI.click(findTestObject('Object Repository/Checking_Customer_Fields/Page_DASHBOARD/a_CUSTOMER EDIT'))

WebUI.setText(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/input_National IDBRNPP No_generalCriteria'), 
    'C9382359332460')

WebUI.click(findTestObject('Checking_Customer_Fields/Page_miFIN/input_National IDBRNPP No_search_1'))

WebUI.click(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/a_CNCIMF000004034'))

WebUI.click(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/a_APPLICANT DETAIL'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/select_SELECT BRNNATIONAL IDPASSPORTREG NUMBER'), 
    '1000000002', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/select_SELECTMAURITIANFOREIGNERMAURITIAN NO_9f4afa'), 
    'FOREIGNER', true)

WebUI.setText(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/input_Passport No_passportNo'), '3451')

WebUI.selectOptionByValue(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/select_SELECT  AFGHANISTANALBANIAALGERIAAME_60d157'), 
    '1000000018', true)

WebUI.click(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/input_Passport Expiry_identificationPassportExpiry'))

WebUI.setText(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/input_Passport Expiry_identificationPassportExpiry'), 
    '02-SEP-2025')

WebUI.click(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/a_Save'))

WebUI.acceptAlert(FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/div_HIGH_tgl_nbtn'))

String customerStatus = WebUI.getText(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/div_APPROVED'))

System.out.println('Customer Status is: ' + customerStatus)

WebUI.click(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/a_APPLICANT'))

WebUI.click(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/a_FINANCIAL  OTHER INFO'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/select_SELECT        SALARIEDSELF EMPLOYED _9c11a1'), 
    '1200000019', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/select_SELECT        SALARIEDSELF EMPLOYED _9c11a1_1'), 
    '1000000014', true)

WebUI.click(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/input_Prof. Inc.  Basic Sal.  Declared Inc__4e9d9c'))

WebUI.doubleClick(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/input_Prof. Inc.  Basic Sal.  Declared Inc__4e9d9c'))

WebUI.setText(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/input_Prof. Inc.  Basic Sal.  Declared Inc__4e9d9c'), 
    '4000')

WebUI.click(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/input_Rental Inc.  Fixed Allowances_rentalIncome'))

WebUI.doubleClick(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/input_Rental Inc.  Fixed Allowances_rentalIncome'))

WebUI.setText(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/input_Rental Inc.  Fixed Allowances_rentalIncome'), 
    '100')

WebUI.click(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/div_INCOME DETAILS (MONTHLY) (IN RS.)'))

WebUI.click(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/a_Save_1'))

WebUI.acceptAlert()

WebUI.click(findTestObject('Checking_Customer_Fields/Financial_Other_Info/Page_miFIN/div_HIGH_tgl_nbtn'))

String financialcustomerStatus = WebUI.getText(findTestObject('Checking_Customer_Fields/Financial_Other_Info/Page_miFIN/div_PENDING'))

System.out.println('Customer Status: ' + financialcustomerStatus)

WebUI.click(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/a_APPLICANT'))

WebUI.click(findTestObject('Checking_Customer_Fields/DM_Quotation/Page_miFIN/div_LEASE RESIDUAL VALUE UPLOAD_toggle-button'))

WebUI.click(findTestObject('Checking_Customer_Fields/DM_Quotation/Page_miFIN/i_VIEWER_img1200004000'))

WebUI.click(findTestObject('Checking_Customer_Fields/DM_Quotation/Page_miFIN/i_DM APPLICATION_img1200004003'))

WebUI.scrollToElement(findTestObject('Checking_Customer_Fields/DM_Quotation/Page_miFIN/a_DM QUOTATION'), 0)

WebUI.click(findTestObject('Checking_Customer_Fields/DM_Quotation/Page_miFIN/a_DM OFFLINE ACTION'))

WebUI.setText(findTestObject('Checking_Customer_Fields/DM_Quotation/Page_miFIN/input_National IDBRNPP No_generalFilter_3_4'), 
    'C9382359332460')

WebUI.click(findTestObject('Checking_Customer_Fields/DM_Quotation/Page_miFIN/input_Engine No_search_1(2)'))

WebUI.click(findTestObject('Checking_Customer_Fields/DM_Quotation/Page_miFIN/a_DMFIN1000008554'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

System.out.println('Customer is not approved')

WebUI.click(findTestObject('Checking_Customer_Fields/DM_Quotation/Page_miFIN/div_LEASE RESIDUAL VALUE UPLOAD_toggle-button'))

WebUI.click(findTestObject('Checking_Customer_Fields/Customer_Dedupe/Page_miFIN/i_CHANGE PASSWORD_img1000002041'))

WebUI.click(findTestObject('Checking_Customer_Fields/Customer_Dedupe/Page_miFIN/i_CUSTOMER_img1000010288'))

WebUI.click(findTestObject('Checking_Customer_Fields/Customer_Dedupe/Page_miFIN/a_DEDUPE'))

WebUI.setText(findTestObject('Checking_Customer_Fields/Customer_Dedupe/Page_miFIN/input_National IDBRNPP No_generalCriteria'), 
    'C9382359332460')

WebUI.click(findTestObject('Checking_Customer_Fields/Customer_Dedupe/Page_miFIN/input_National IDBRNPP No_search'))

WebUI.click(findTestObject('Checking_Customer_Fields/Customer_Dedupe/Page_miFIN/a_CNCIMF99999909'))

WebUI.click(findTestObject('Checking_Customer_Fields/Customer_Dedupe/Page_LOAN LIFECYCLE MANAGER/a_APPLICANT'))

WebUI.waitForElementVisible(findTestObject('Object Repository/Page_miFIN/input_N_reInitiate'), 4, FailureHandling.OPTIONAL)

/* Financial and Other info ends here */
WebUI.click(findTestObject('Object Repository/Page_miFIN/input_N_reInitiate'))

WebUI.click(findTestObject('Object Repository/Page_miFIN/a_DEDUPE'))

/*Dedupe starts below */
WebUI.click(findTestObject('Object Repository/dedupe/dedupe_plus_btn'))

/* A1 starts - Capturing reference number test 

GlobalVariable.custID = WebUI.getText(findTestObject('Object Repository/dedupe/dedupe_cust_ref_num'))

System.out.println(GlobalVariable.custID)


 Logic A1- ends here */
WebUI.click(findTestObject('Checking_Customer_Fields/Page_LOAN LIFECYCLE MANAGER/a_CNCIMF000004060_1'))

WebUI.click(findTestObject('Object Repository/dedupe/a_Save'))

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/New cust creation flow/a_DOCUMENT'))

WebUI.waitForPageLoad(3, FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Checking_Customer_Fields/Customer_Document/Page_miFIN/input_Last Updated Date_updated'))

WebUI.setText(findTestObject('Checking_Customer_Fields/Customer_Document/Page_miFIN/textarea_Last Updated Date_remarks'), 
    'OK')

WebUI.selectOptionByValue(findTestObject('Object Repository/dedupe/select_SELECTRECEIVEDPENDINGDEFERRED'), '1000000001', 
    true)

/*CR Review*/
WebUI.click(findTestObject('Object Repository/New cust creation flow/a_CR REVIEW'))

WebUI.selectOptionByValue(findTestObject('Object Repository/New cust creation flow/select_SELECTREJECT RECOMMEND'), '1000000003', 
    true)

WebUI.setText(findTestObject('Object Repository/New cust creation flow/textarea_Remarks_remark'), 'ok')

WebUI.click(findTestObject('Object Repository/dedupe/a_Save_1'))

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('New cust creation flow/Page_miFIN/img_Hi COPSUSERM_userr_1'))

WebUI.click(findTestObject('New cust creation flow/Page_miFIN/a_Logout_1'))

WebUI.setText(findTestObject('Checking_Customer_Fields/Page_miFIN/input_miFIN_userId'), 'FARZANAN')

WebUI.setEncryptedText(findTestObject('Object Repository/Checking_Customer_Fields/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Checking_Customer_Fields/Page_miFIN/button_LOGIN'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/CR Approval/i_CHANGE PASSWORD_img1000002041'))

WebUI.maximizeWindow()

WebUI.click(findTestObject('Object Repository/CR Approval/i_CUSTOMER_img1000010288'))

WebUI.click(findTestObject('Object Repository/CR Approval/a_CR REVIEW'))

WebUI.setText(findTestObject('Checking_Customer_Fields/Page_miFIN/input_National IDBRNPP No_generalCriteria_2'), C9382359332460)

WebUI.click(findTestObject('Object Repository/CR Approval/input_National IDBRNPP No_search'))

ApplicationCode = WebUI.getText(findTestObject('CR Approval/a_CNCIMF000002263'))

System.out.println(ApplicationCode)

WebUI.click(findTestObject('Object Repository/CR Approval/a_CNCIMF000002263'))

WebUI.selectOptionByValue(findTestObject('Object Repository/CR Approval/select_SELECTREJECT APPROVE SEND BACK TO PR_0bbc91'), 
    '1000000004', true)

WebUI.setText(findTestObject('Object Repository/CR Approval/textarea_Remarks_remark_1'), 'ok')

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/additional items/a_Save'))

WebUI.waitForAlert(2, FailureHandling.OPTIONAL)

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/CR Approval/img_Hi FARZANAN_userr'))

WebUI.click(findTestObject('Page_miFIN/a_Logout (1)'))

WebUI.closeBrowser()

