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

WebUI.setText(findTestObject('Object Repository/all in one/input_miFIN_userId'), 'navind')

WebUI.setEncryptedText(findTestObject('Object Repository/all in one/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/all in one/button_LOGIN'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.maximizeWindow()

WebUI.click(findTestObject('Object Repository/all in one/i_GROUP LIMIT VIEWER_img1200003000'))

WebUI.click(findTestObject('Object Repository/all in one/i_QUOTATION_img1200003001'))

WebUI.click(findTestObject('Object Repository/all in one/a_LEASE PROPOSAL'))

WebUI.click(findTestObject('Object Repository/all in one/input_Entity Code_btn btn-primary btn-sm'))

WebUI.waitForPageLoad(3)

WebUI.closeWindowTitle('miFIN')

WebUI.switchToWindowUrl('https://mifinuat.cim.local/lease/quotationSearchAction.do?actionPerformed=displaySearchScreen&searchType=CUSTOMER&screenFlag=Y&parentBodyId=quotationNewApplicantId')

/* to link below to excel */
WebUI.setText(findTestObject('Object Repository/all in one/input_CUSTOMERCOMPANY FNAME_firstName'), first_name)

WebUI.setText(findTestObject('Object Repository/all in one/input_LAST NAME_lastName'), last_name)

WebUI.click(findTestObject('Object Repository/all in one/input_Entity Code_btn btn-primary btn-sm'))

WebUI.click(findTestObject('Object Repository/all in one/input_MOBILE_selectApplicant'))

WebUI.click(findTestObject('Object Repository/all in one/input_ROHAN TESTQA_btn btn-primary btn-sm'))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/all in one/input_Entity Code_btn btn-primary btn-sm_1'))

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTHEAD OFFICE'), '1000000037', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTALLY OOZEERALLYFABRICE DANGEOT_bfa801'), 
    '1000000001', true)

WebUI.click(findTestObject('Object Repository/Page_miFIN/a_Save  Continue'))

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

/*ASSET DETAILS*/
WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTFLOATING FIN LEASE CORPORATE F_3c5fb7'), 
    '1200000112', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTONLY FIXED FINANCE LEASEONLY F_9537c6'), 
    '1200000140', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECT16EME MILLEAFGHANISTANALBANIAA_a07ba9_1'), 
    'string:1000000211', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTANSE COURTOISBAIN DES DAMESBAN_49ffce'), 
    'string:22656', true)

WebUI.click(findTestObject('Object Repository/all in one/i_ASSET DETAILS_fa fa-plus'))

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTEQUIPMENTVEHICLE'), 'string:1000000007', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTEQUIPMENTVEHICLE_1'), 'string:1000000028', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTTRAIN ROULANTPORSCHEJINBEIDIAH_049b74'), 
    'string:1000000166', true)

WebUI.sendKeys(findTestObject('Object Repository/all in one/input_Advance Search_advanceSearchID0'), 'land')

WebUI.sendKeys(findTestObject('Object Repository/all in one/input_Advance Search_advanceSearchID0'), Keys.chord(Keys.ENTER))

WebUI.sendKeys(findTestObject('Object Repository/all in one/input_Advance Search_advanceSearchID0'), Keys.chord(Keys.DOWN))

WebUI.sendKeys(findTestObject('Object Repository/all in one/input_Advance Search_advanceSearchID0'), Keys.chord(Keys.ENTER))

WebUI.setText(findTestObject('Object Repository/all in one/input_Yr of Manuf_MANUFACTURING_YEAR0'), '2020')

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTAFGHANISTANALBANIAALGERIAAMERI_5225cc'), 
    '1000000020', true)

WebUI.doubleClick(findTestObject('Object Repository/all in one/input_Ex-Show Room_EXSHOWROOM_COST0'))

WebUI.sendKeys(findTestObject('Object Repository/all in one/input_Ex-Show Room_EXSHOWROOM_COST0'), '100')

WebUI.setText(findTestObject('Object Repository/all in one/input_Remarks_REMARKS0'), 'ok')

WebUI.click(findTestObject('Object Repository/all in one/input_ADDITIONAL CHARGE DETAILS_btn btn-pri_82522f'))

WebUI.click(findTestObject('Object Repository/all in one/input_ADDITIONAL CHARGE DETAILS_btn btn-pri_82522f'))

WebUI.click(findTestObject('Object Repository/all in one/input_ADDITIONAL CHARGE DETAILS_btn btn-pri_82522f'))

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTDEPOSIT PAID AT CIMVAT ON EX-S_44a36b'), 
    '1200003001', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTDEPOSIT PAID AT CIMVAT ON EX-S_44a36b_1'), 
    '1200003002', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTDEPOSIT PAID AT CIMVAT ON EX-S_44a36b_1_2'), 
    '1200003003', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTRENTALISEOPE TO LESSEBY LESSE'), '1000000002', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTRENTALISEOPE TO LESSEBY LESSE_1'), '1000000002', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTRENTALISEOPE TO LESSEBY LESSE_1_2'), 
    '1000000002', true)

WebUI.click(findTestObject('Object Repository/all in one/input_Dealer Code_CHARGE_AMOUNT2'))

WebUI.doubleClick(findTestObject('Object Repository/all in one/input_Dealer Code_CHARGE_AMOUNT2'))

WebUI.sendKeys(findTestObject('Object Repository/all in one/input_Dealer Code_CHARGE_AMOUNT2'), '100')

WebUI.click(findTestObject('Object Repository/all in one/input_Dealer Code_CHARGE_AMOUNT3'))

WebUI.doubleClick(findTestObject('Object Repository/all in one/input_Dealer Code_CHARGE_AMOUNT3'))

WebUI.sendKeys(findTestObject('Object Repository/all in one/input_Dealer Code_CHARGE_AMOUNT3'), '100')

WebUI.scrollToElement(findTestObject('Object Repository/all in one/input_Dealer Code_CHARGE_AMOUNT3'), 0)

WebUI.click(findTestObject('Object Repository/all in one/input_Dealer Code_CHARGE_AMOUNT4'))

WebUI.doubleClick(findTestObject('Object Repository/all in one/input_Dealer Code_CHARGE_AMOUNT4'))

WebUI.sendKeys(findTestObject('Object Repository/Page_miFIN/input_Dealer Code_CHARGE_AMOUNT4'), '200')

WebUI.click(findTestObject('Object Repository/all in one/input_Dealer Code_btndealerName'))

WebUI.switchToWindowTitle('DealerChooser')

/* Changes made below, delior as dealer and GOVT for registration duty */
WebUI.setText(findTestObject('Object Repository/all in one/input_Name_populateId'), 'Delior')

WebUI.sendKeys(findTestObject('Object Repository/all in one/input_Name_populateId'), Keys.chord(Keys.ENTER))

WebUI.sendKeys(findTestObject('Object Repository/all in one/input_Name_populateId'), Keys.chord(Keys.ENTER))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/all in one/input_Dealer Code_btndealerName_1'))

WebUI.switchToWindowTitle('DealerChooser')

WebUI.setText(findTestObject('Object Repository/all in one/input_Name_populateId'), 'Delior')

WebUI.sendKeys(findTestObject('Object Repository/all in one/input_Name_populateId'), Keys.chord(Keys.ENTER))

WebUI.sendKeys(findTestObject('Object Repository/all in one/input_Name_populateId'), Keys.chord(Keys.ENTER))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/all in one/input_Dealer Code_btndealerName_1_2'))

WebUI.switchToWindowTitle('DealerChooser')

WebUI.setText(findTestObject('Object Repository/all in one/input_Name_populateId'), 'Government of Mauritius')

WebUI.sendKeys(findTestObject('Object Repository/all in one/input_Name_populateId'), Keys.chord(Keys.ENTER))

WebUI.sendKeys(findTestObject('Object Repository/all in one/input_Name_populateId'), Keys.chord(Keys.ENTER))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

WebUI.acceptAlert()

WebUI.waitForPageLoad(5)

/* Quotation Screen */
WebUI.click(findTestObject('Object Repository/all in one/a_QUOTATION'))

WebUI.waitForPageLoad(5)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTEQUATEDINTEREST ONLY_1'), 'string:1000000001', 
    true)

WebUI.click(findTestObject('Object Repository/all in one/input_Lease Period(Months)_lease_Period'))
WebUI.click(findTestObject('Object Repository/all in one/select_SELECTMONTHLY'))

WebUI.sendKeys(findTestObject('Object Repository/all in one/select_SELECTMONTHLY'), 'Monthly')

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECT17142128'), '7', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECT9911001'), '1000000002', true)
		
WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTTRI PARTYBI PARTY'), 'string:1000000001',
			true)

WebUI.click(findTestObject('Object Repository/all in one/input_Lease Period(Months)_lease_Period'))

WebUI.doubleClick(findTestObject('Object Repository/all in one/input_Lease Period(Months)_lease_Period'))

WebUI.sendKeys(findTestObject('Object Repository/all in one/input_Lease Period(Months)_lease_Period'), '10')

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECT9911001'), '1000000002', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTEX-SHOWROOMEX-SHOWROOM LESS DI_3e0470'), 
    '1000000001', true)

WebUI.click(findTestObject('Object Repository/all in one/input_RVBuy Back_fixed_rv'))

WebUI.doubleClick(findTestObject('Object Repository/all in one/input_RVBuy Back_fixed_rv'))

WebUI.sendKeys(findTestObject('Object Repository/all in one/input_RVBuy Back_fixed_rv'), '1.00')

WebUI.sendKeys(findTestObject('Object Repository/all in one/input_RVBuy Back_fixed_rv'), Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/all in one/a_CASHFLOW'))

WebUI.click(findTestObject('Object Repository/all in one/input_ROI_CASH_FLOW_IRR'))

WebUI.doubleClick(findTestObject('Object Repository/all in one/input_ROI_CASH_FLOW_IRR'))

WebUI.setText(findTestObject('Object Repository/all in one/input_ROI_CASH_FLOW_IRR'), '5.00')

WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/all in one/a_REGISTRATION DETAILS'))

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTDEBARCADEREHOSPITALILE AUX DEU_f9afde'), 
    '22227', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTFLOATING FIN LEASE CORPORATE F_3c5fb7'), 
    '1200000112', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTONLY FIXED FINANCE LEASEONLY F_ebb241'), 
    '1200000140', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTLESSEEUSERCO-LESSEEBUYEROTHERS_47e6ef'), 
    '1000000001', true)

WebUI.click(findTestObject('Object Repository/Page_miFIN/input_ADDRESS INFORMATION_btn btn-primary btn-sm'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECTRESIDENCE ADDRESSOFFICE ADDRES_5733be'), 
    '1200000009', true)

WebUI.setText(findTestObject('Object Repository/all in one/input_Address 1_FLATNO'), 'royal road port-louis')

WebUI.click(findTestObject('Object Repository/all in one/input_Pincode_btnZip'))

WebUI.switchToWindowTitle('Pincode')

WebUI.click(findTestObject('Object Repository/all in one/font_C_H_ Leal Lane'))

WebUI.click(findTestObject('Object Repository/all in one/input_Impasse Demerez_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.waitForElementPresent(findTestObject('Object Repository/all in one/a_Save'), 5)

WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/all in one/a_ACCEPTANCE'))

WebUI.click(findTestObject('Object Repository/all in one/td_SELECTREJECTAPPROVE'))

WebUI.selectOptionByValue(findTestObject('Object Repository/all in one/select_SELECTREJECTAPPROVE'), 'string:1000000004', 
    true)

WebUI.setText(findTestObject('Object Repository/all in one/input_REMARKS_REMARKS'), 'ok')

WebUI.click(findTestObject('Object Repository/all in one/a_Save'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/Page_miFIN/img_Hi NAVIND_userr'))

WebUI.click(findTestObject('Object Repository/Page_miFIN/a_Logout (2)'))

WebUI.closeBrowser()

