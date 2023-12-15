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

WebUI.deleteAllCookies(FailureHandling.OPTIONAL)

WebUI.navigateToUrl('https://mifinuat.cim.local/lease/')

WebUI.setText(findTestObject('New cust creation flow/input_miFIN_userId'), 'Copsuserm')

WebUI.setEncryptedText(findTestObject('New cust creation flow/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/Page_miFIN/button_LOGIN (1)'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/Page_DASHBOARD/i_CHANGE PASSWORD_img1000002041'))

WebUI.maximizeWindow()

WebUI.click(findTestObject('Object Repository/Page_DASHBOARD/i_CUSTOMER_img1000010289'))

WebUI.click(findTestObject('Object Repository/Page_DASHBOARD/a_CUSTOMER'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTNEWEXISTINGDORMANTINACTIVE'), '1000000001', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTMSMRMRSSIRMISS'), '1000000002', true)

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_First Name_firstName_1_2_3_4_5_6_7'), first_name)

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_Last Name_lastName_1_2_3_4'), last_name)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTMARRIEDSINGLEDIVORCEDWIDOWEDSEPARATED'), 
    '1000000002', true)

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_Date of Birth_dob_1_2_3_4_5_6_7_8_9_10_11'), date_of_birth)

WebUI.sendKeys(findTestObject('Object Repository/Page_miFIN/select_SELECTMALEFEMALEOTHER'), 'MALE')

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_National ID_aadhaarNo_1_2_3_4_5_6_7_8_9_10_11_12_13_14'), 
    nic_num)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTAGRICULTURAL AND FISHERY SKILL_894a60'), 
    '5', true)

/*
 
WebUI.click(findTestObject('Object Repository/Page_Industry/input_Industry_btnIndustry'))

WebUI.switchToWindowTitle('Industry')

WebUI.click(findTestObject('Object Repository/Page_Industry/font_A-AGRICULTURE FORESTRY AND FISHING'))

WebUI.click(findTestObject('Object Repository/Page_Industry/input_U-ACTIVITIES OF EXTRATERRITORIAL ORGANIZATIONS AND BODIES_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.closeWindowTitle('miFIN')

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/Page_Sector/input_Sector_btnIndustry'))

WebUI.switchToWindowTitle('Sector')

WebUI.click(findTestObject('Object Repository/Page_Sector/font_A.011-Growing of non-perennial crops(Seasonal)'))

WebUI.click(findTestObject('Object Repository/Page_Sector/input_U.9900-Activities of extraterritorial organizations and bodies_OK'))

WebUI.switchToWindowTitle('miFIN')

*/
/* New Flow as per divya */
WebUI.click(findTestObject('Object Repository/Page_miFIN/input_Employer Name_btnemployer'))

WebUI.switchToWindowTitle('EmployerName')

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_Name_populateId'), 'divya')

WebUI.sendKeys(findTestObject('Object Repository/Page_miFIN/input_Name_populateId'), Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Page_miFIN/td_1000061687'))

WebUI.click(findTestObject('Object Repository/Page_miFIN/font_DIVYA'))

WebUI.click(findTestObject('Object Repository/Page_miFIN/input_DIVYAPHAL INVESTMENT_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.closeWindowTitle('miFIN')

WebUI.switchToWindowTitle('miFIN')

/* Changes ends here - employer Name */
WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_Birth Certificate No_personalBirthCer_e64c32_1_2_3_4_5_6'), 
    birth_cer_num)

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_Civil Status Office_personalCivilStatus_1_2_3_4_5_6'), 
    civil_status_office)

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_Place of Birth_personalPlaceOfBirth_1_2_3_4_5_6_7_8_9_10_11_12_13'), 
    place_of_birth)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTHEAD OFFICESUB OFFICE CUREPIPE_919cd3'), 
    '1000000037', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTALLY OOZEERALLYASER GRACE DHOO_b0ef4e'), 
    '1000000001', true)

WebUI.takeScreenshot()

WebUI.scrollToElement(findTestObject('Object Repository/Others/input_State_btnState'), 0)

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_Address1_flatNo_1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16_17_18_19_20_21'), 
    address_1)

WebUI.click(findTestObject('Object Repository/Others/input_Country_btnState'))

WebUI.switchToWindowTitle('Country')

WebUI.setText(findTestObject('Object Repository/Others/input_Name_populateId'), 'mauritius')

WebUI.click(findTestObject('Object Repository/Others/font_MAURITIUS'))

WebUI.click(findTestObject('Object Repository/Others/input_MAURITIUS_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.scrollToElement(findTestObject('Object Repository/Others/input_State_btnState'), 0)

WebUI.click(findTestObject('Object Repository/Others/input_State_btnState'))

WebUI.switchToWindowTitle('State')

WebUI.setText(findTestObject('Object Repository/Others/input_Name_populateId (1)'), 'port')

WebUI.click(findTestObject('Object Repository/Others/font_PORT LOUIS'))

WebUI.click(findTestObject('Object Repository/Others/input_RODRIGUES - PORT MATHURIN_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/Others/input_City_btnCity'))

WebUI.switchToWindowTitle('City')

WebUI.setText(findTestObject('Object Repository/Others/input_Name_populateId (2)'), 'cas')

WebUI.click(findTestObject('Object Repository/Others/font_CASSIS 1'))

WebUI.click(findTestObject('Object Repository/Others/input_CASSIS 3_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/Others/input_Pincode_btnzipPin'))

WebUI.switchToWindowTitle('Pincode')

WebUI.click(findTestObject('Object Repository/Others/font_C_H_ Leal Lane'))

WebUI.click(findTestObject('Object Repository/Others/input_Impasse Demerez_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_Mobile No_mobile_1_2_3_4_5_6_7_8'), mobile_num)

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_Landline_std_1_2_3'), '230')

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_Landline_landLine_1_2_3_4_5_6_7_8'), '12345678')

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_E-Mail_email'), 'testqa@gmail.com')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTSELFOWNEDRENTEDPARENTALIN LAWS_0e261d'), 
    '1000000001', true)

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_YearsMonth_noOfYearAtResidence_1_2'), '10')

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_YearsMonth_noOfMonthAtResidence'), '10')

WebUI.waitForElementClickable(findTestObject('New cust creation flow/Page_miFIN/a_Save  Continue_updated'), 3, FailureHandling.OPTIONAL)

WebUI.click(findTestObject('New cust creation flow/Page_miFIN/a_Save  Continue_updated'))

WebUI.acceptAlert(FailureHandling.STOP_ON_FAILURE)

WebUI.takeScreenshot(FailureHandling.OPTIONAL)

/* Applicant details */
WebUI.click(findTestObject('Object Repository/Page_miFIN/a_APPLICANT DETAIL'))

WebUI.click(findTestObject('Object Repository/Page_miFIN/span'))

WebUI.selectOptionByValue(findTestObject('New cust creation flow/Page_miFIN/select_SELECTLEASE (1)'), '1000000003', true)

WebUI.selectOptionByValue(findTestObject('New cust creation flow/Page_miFIN/select_SELECTFINANCE GREEN LEASEFINANCE LEA_3f85bf (1)'), 
    '1200000112', true)

WebUI.selectOptionByValue(findTestObject('New cust creation flow/Page_miFIN/select_SELECTEQUIPMENTVEHICLE (1)'), '1200000318', 
    true)

/*

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTFINANCE GREEN LEASEFINANCE LEA_3f85bf'), 
    '1200000112', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTEQUIPMENTVEHICLE'), '1200000317', true)

*/
WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTCORPORATESMELARGE CORPORATEINDIVIDUAL'), 
    '1000000017', true)

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_Profession (Job Title)_professionIND'), 'TEACHER')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECT BRNNATIONAL IDPASSPORT'), '1000000001', 
    true)

WebUI.click(findTestObject('Object Repository/Page_miFIN/a_Save'))

WebUI.acceptAlert(FailureHandling.STOP_ON_FAILURE)

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Page_miFIN/a_APPLICANT'))

WebUI.waitForElementVisible(findTestObject('Object Repository/Page_miFIN/a_FINANCIAL  OTHER INFO'), 4)

/* Financial and other info */
WebUI.click(findTestObject('Object Repository/Page_miFIN/a_FINANCIAL  OTHER INFO'))

WebUI.setText(findTestObject('Page_miFIN/input_Designation_designation_1_2'), 'HR')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECT                         CURRE_28079a'), 
    '1000000002', true)

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_MICR Code_beneficiaryName_1_2_3_4_5_6_7'), 'TESTING')

WebUI.click(findTestObject('Object Repository/Page_miFIN/input_MICR Code_btnEditBankName'))

WebUI.switchToWindowTitle('Bank')

WebUI.setText(findTestObject('Object Repository/Page_Bank/input_Name_populateId'), 'CIM BANK')

WebUI.click(findTestObject('Object Repository/Others/font_CIM BANK'))

WebUI.click(findTestObject('Object Repository/Page_Bank/input_CIM GREEN LEASE BANK_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/Page_miFIN/input_MICR Code_btnEditBranchName'))

WebUI.switchToWindowTitle('Branch')

WebUI.click(findTestObject('Object Repository/Page_Branch/font_PORT LOUIS'))

WebUI.click(findTestObject('Object Repository/Page_Branch/input_PORT LOUIS_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_MICR Code_accountNameId_1_2_3_4_5_6_7_8'), '12345678')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTUSDGBPEURMUR'), '1000000001', true)

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_No. of Years in ServiceBusiness_curre_d7860b_1_2'), '12')

WebUI.scrollToElement(findTestObject('Object Repository/Page_Industry/input_Household Expenses_householdExpenses'), 0)

WebUI.doubleClick(findTestObject('Object Repository/Page_Industry/input_Prof. Inc.  Basic Sal.  Declared Inc__4e9d9c'))

WebUI.setText(findTestObject('Object Repository/Page_Industry/input_Prof. Inc.  Basic Sal.  Declared Inc__4e9d9c'), '100000')

WebUI.doubleClick(findTestObject('Object Repository/Page_Industry/input_Rental Inc.  Fixed Allowances_rentalIncome'))

WebUI.setText(findTestObject('Object Repository/Page_Industry/input_Rental Inc.  Fixed Allowances_rentalIncome'), '1000')

WebUI.setText(findTestObject('Page_Industry/input_Interest  Disc. Inc.  Other Allow_int_6b23af'), '100')

WebUI.click(findTestObject('Object Repository/Page_Industry/input_Other Income_otherIncomeDetail'))

WebUI.doubleClick(findTestObject('Object Repository/Page_Industry/input_Other Income_otherIncomeDetail'))

WebUI.setText(findTestObject('Object Repository/Page_Industry/input_Other Income_otherIncomeDetail'), '2000')

WebUI.setText(findTestObject('Object Repository/Page_Industry/input_Professional Expenses  Deductions_pro_fda2de'), '5000')

WebUI.setText(findTestObject('Object Repository/Page_Industry/input_Household Expenses_householdExpenses'), '20000')

WebUI.scrollToElement(findTestObject('Object Repository/Page_miFIN/a_APPLICANT'), 0)

WebUI.click(findTestObject('Object Repository/Page_miFIN/a_Save_1'))

WebUI.waitForAlert(2, FailureHandling.OPTIONAL)

WebUI.acceptAlert()

WebUI.waitForAlert(2, FailureHandling.OPTIONAL)

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Page_miFIN/a_APPLICANT'))

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
WebUI.click(findTestObject('Object Repository/dedupe/dedupe_cust_ref_num'))

WebUI.click(findTestObject('Object Repository/dedupe/a_Save'))

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.waitForElementVisible(findTestObject('Object Repository/dedupe/a_NOTEPAD'), 3, FailureHandling.OPTIONAL)

/* Notepad */
WebUI.click(findTestObject('Object Repository/dedupe/a_NOTEPAD'))

WebUI.selectOptionByValue(findTestObject('Object Repository/dedupe/select_SELECTDATA ENTRYAPPLICANT ENTRYVERIF_e6d432'), 
    '1000000001', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/dedupe/select_SELECTNOTECVNOTEAUSER NOTECOLLN NOTE_7d6e22'), 
    '1000000001', true)

WebUI.setText(findTestObject('Object Repository/dedupe/textarea_Description_description_1'), 'ok')

WebUI.setText(findTestObject('Object Repository/dedupe/textarea_Remarks_remarks_1'), 'ok')

WebUI.click(findTestObject('Object Repository/dedupe/a_Save_1'))

/* DOcuments */
WebUI.click(findTestObject('Object Repository/New cust creation flow/a_DOCUMENT'))

WebUI.waitForPageLoad(3, FailureHandling.OPTIONAL)

WebUI.click(findTestObject('dedupe/input_Select Document_updated'))

WebUI.click(findTestObject('dedupe/input_SYSUSER_updated'))

WebUI.click(findTestObject('dedupe/input_SYSUSER_updated_1'))

WebUI.setText(findTestObject('Object Repository/dedupe/textarea_Select Document_remarks_1'), 'ok')

WebUI.setText(findTestObject('Object Repository/dedupe/textarea_SYSUSER_remarks'), 'ok')

WebUI.setText(findTestObject('Object Repository/dedupe/textarea_SYSUSER_remarks_1_2'), 'ok')

WebUI.selectOptionByValue(findTestObject('Object Repository/dedupe/select_SELECTRECEIVEDPENDINGDEFERRED'), '1000000001', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/dedupe/select_SELECTRECEIVEDPENDINGDEFERRED_1'), '1000000001', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/dedupe/select_SELECTRECEIVEDPENDINGDEFERRED_1_2'), '1000000001', 
    true)

WebUI.uploadFile(findTestObject('New cust creation flow/Page_miFIN/input_Last Updated Date_file'), file_kyc)

WebUI.uploadFile(findTestObject('New cust creation flow/Page_miFIN/input_SYSUSER_file_1'), file_kyc)

WebUI.uploadFile(findTestObject('New cust creation flow/Page_miFIN/input_SYSUSER_file_2'), file_kyc)

WebUI.click(findTestObject('New cust creation flow/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.waitForElementVisible(findTestObject('Object Repository/New cust creation flow/a_CR REVIEW'), 0, FailureHandling.OPTIONAL)

/*CR Review*/
WebUI.click(findTestObject('Object Repository/New cust creation flow/a_CR REVIEW'))

WebUI.selectOptionByValue(findTestObject('Object Repository/New cust creation flow/select_SELECTREJECT RECOMMEND'), '1000000003', 
    true)

WebUI.setText(findTestObject('Object Repository/New cust creation flow/textarea_Remarks_remark'), 'ok')

WebUI.click(findTestObject('Object Repository/dedupe/a_Save_1'))

WebUI.acceptAlert()

WebUI.waitForAlert(2, FailureHandling.OPTIONAL)

WebUI.acceptAlert(FailureHandling.OPTIONAL)

/* CR Review ends here */
/* Extracting Customer ID reference to Global Variable 

WebUI.click(findTestObject('Object Repository/Extract Texts/div_TOP info btn'))

GlobalVariable.custID = WebUI.getText(findTestObject('Object Repository/Extract Texts/div_custID'))

System.out.println(GlobalVariable.custID)

 END verify if CustID is extracted in console */
WebUI.click(findTestObject('Object Repository/logout/img_Hi NAVINS_userr'))

WebUI.click(findTestObject('logout/a_Logout'))

WebUI.switchToWindowTitle('miFIN', FailureHandling.STOP_ON_FAILURE)

