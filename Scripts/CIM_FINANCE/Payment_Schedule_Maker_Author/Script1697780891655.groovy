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

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_miFIN_userId (3)'), 'COPSUSERM')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_miFIN/input_miFIN_password (3)'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/Page_miFIN/button_LOGIN (2)'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.maximizeWindow()

WebUI.click(findTestObject('Object Repository/Page_DASHBOARD/i_VIEWER_img1200004000 (1)'))

WebUI.click(findTestObject('Object Repository/Page_DASHBOARD/i_DM APPLICATION_img1200004003 (1)'))

WebUI.click(findTestObject('Object Repository/Page_DASHBOARD/a_DM OFFLINE ACTION (1)'))

WebUI.setText(findTestObject('Payment/Page_miFIN/input_National IDBRNPP No_generalFilter'), nic_num)

WebUI.click(findTestObject('Object Repository/Page_miFIN/input_Engine No_search (2)'))

String dmCode = WebUI.getText(findTestObject('Object Repository/Page_miFIN/a_DMFIN1000008427'))

WebUI.click(findTestObject('Object Repository/Page_miFIN/a_DMFIN1000008427'))

WebUI.scrollToElement(findTestObject('Payment/Payment_Schedule/Page_miFIN/div_Dealer Name'), 0)

WebUI.selectOptionByIndex(findTestObject('Payment/Payment_Schedule/Page_miFIN/select_SELECTTHE CAR CONNEXION LTD'), 1, FailureHandling.STOP_ON_FAILURE)

WebUI.selectOptionByValue(findTestObject('Payment/Payment_Schedule/Page_miFIN/select_SELECTCHEQUERTGSBANK TRANSFERCASHDEM_da6496'), 
    '1000000002', true)

WebUI.click(findTestObject('Object Repository/Payment/Payment_Schedule/Page_miFIN/input_Instrument Type_btn btn-primary btn-sm'))

WebUI.selectOptionByIndex(findTestObject('Payment/Payment_Schedule/Page_miFIN/select_SELECTDELIOR LTDGOVERNMENT OF MAURITIUS_1'), 
    2, FailureHandling.STOP_ON_FAILURE)

WebUI.selectOptionByValue(findTestObject('Payment/Payment_Schedule/Page_miFIN/select_SELECTCHEQUERTGSBANK TRANSFERCASHDEM_da6496'), 
    '1000000002', true)

WebUI.click(findTestObject('Object Repository/Payment/Payment_Schedule/Page_miFIN/input_Instrument Type_btn btn-primary btn-sm'))

WebUI.click(findTestObject('Object Repository/Payment/Payment_Schedule/Page_miFIN/input_OperationsUser_ROW_SELECT0'))

WebUI.click(findTestObject('Object Repository/Payment/Payment_Schedule/Page_miFIN/input_OperationsUser_SCHEDULE_PAYMENT0'))

String paymentDate = WebUI.getText(findTestObject('Payment/Page_miFIN/span_01-AUG-2023'))

WebUI.setText(findTestObject('Payment/Payment_Schedule/Page_miFIN/input_OperationsUser_PAYMENT_DATE0'), paymentDate)

WebUI.click(findTestObject('Payment/Payment_Schedule/Page_miFIN/input_OperationsUser_ROW_SELECT1'), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Payment/Payment_Schedule/Page_miFIN/input_OperationsUser_SCHEDULE_PAYMENT1'), FailureHandling.OPTIONAL)

WebUI.setText(findTestObject('Payment/Payment_Schedule/Page_miFIN/input_OperationsUser_PAYMENT_DATE1'), paymentDate, FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Payment/Payment_Schedule/Page_miFIN/input_OperationsUser_ROW_SELECT2'), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Payment/Payment_Schedule/Page_miFIN/input_OperationsUser_SCHEDULE_PAYMENT2'), FailureHandling.OPTIONAL)

WebUI.setText(findTestObject('Payment/Payment_Schedule/Page_miFIN/input_OperationsUser_PAYMENT_DATE2'), paymentDate, FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Payment/Payment_Schedule/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.click(findTestObject('Payment/Payment_Maker/Page_miFIN/div_LEASE RESIDUAL VALUE UPLOAD_toggle-button'))

WebUI.scrollToElement(findTestObject('Payment/Payment_Maker/Page_DASHBOARD/a_DM QUOTATION'), 2)

WebUI.click(findTestObject('Object Repository/Payment/Payment_Maker/Page_miFIN/i_VIEWER_img1200004029'))

WebUI.click(findTestObject('Object Repository/Payment/Payment_Maker/Page_miFIN/a_MAKER'))

WebUI.click(findTestObject('Object Repository/Payment/Payment_Maker/Page_miFIN/input_Prospect Code_colProspectCode'))

WebUI.setText(findTestObject('Object Repository/Payment/Payment_Maker/Page_miFIN/input_Prospect Code_colProspectCode_1'), 
    dmCode)

WebUI.click(findTestObject('Object Repository/Payment/Payment_Maker/Page_miFIN/input_Payment To_searchBtn'))

WebUI.click(findTestObject('Payment/Payment_Maker/Page_miFIN/input_Status_payment_Row'))

WebUI.setText(findTestObject('Payment/Payment_Maker/Page_miFIN/input_DMFIN1000008427_instDate'), paymentDate)

WebUI.setText(findTestObject('Payment/Payment_Maker/Page_miFIN/input_DMFIN1000008427_mRemarks'), 'ok')

WebUI.click(findTestObject('Payment/Payment_Maker/Page_miFIN/input_DMFIN1000008577_payment_Row'))

WebUI.setText(findTestObject('Payment/Payment_Maker/Page_miFIN/input_DMFIN1000008577_instDate'), paymentDate)

WebUI.setText(findTestObject('Payment/Payment_Maker/Page_miFIN/input_DMFIN1000008577_mRemarks_2'), 'ok')

WebUI.scrollToElement(findTestObject('Payment/Payment_Maker/Page_miFIN/font_Send To Author'), 0)

WebUI.click(findTestObject('Payment/Payment_Maker/Page_miFIN/input_Send To Author_mrSendToAuthor'))

WebUI.click(findTestObject('Payment/Payment_Maker/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.waitForPageLoad(5, FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Payment/Payment_Maker/Page_miFIN/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Payment/Payment_Maker/Page_miFIN/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Page_miFIN/input_miFIN_userId'), 'FARZANAN')

WebUI.setEncryptedText(findTestObject('Object Repository/Payment/Payment_Author/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/Payment/Payment_Author/Page_miFIN/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/Payment/Payment_Author/Page_DASHBOARD/i_VIEWER_img1200004000'))

WebUI.click(findTestObject('Object Repository/Payment/Payment_Author/Page_DASHBOARD/i_VIEWER_img1200004029'))

WebUI.click(findTestObject('Object Repository/Payment/Payment_Author/Page_DASHBOARD/a_AUTHOR'))

WebUI.setText(findTestObject('Object Repository/Payment/Payment_Author/Page_miFIN/input_Prospect Code_colProspectCode'), 
    dmCode)

WebUI.click(findTestObject('Object Repository/Payment/Payment_Author/Page_miFIN/input_Payment To_searchBtn'))

WebUI.click(findTestObject('Object Repository/Payment/Payment_Author/Page_miFIN/input_Status_payment_Row'))

WebUI.setText(findTestObject('Object Repository/Payment/Payment_Author/Page_miFIN/input_DMFIN1000008427_aRemarks_1'), 'ok')

WebUI.click(findTestObject('Payment/Payment_Author/Page_miFIN/input_DMFIN1000008577_payment_Row'))

WebUI.setText(findTestObject('Payment/Payment_Author/Page_miFIN/input_DMFIN1000008427_aRemarks_1_2'), 'ok')

WebUI.scrollToElement(findTestObject('Payment/Payment_Author/Page_miFIN/div_Approve'), 0)

WebUI.click(findTestObject('Object Repository/Payment/Payment_Author/Page_miFIN/input_Approve_arApprove'))

WebUI.click(findTestObject('Object Repository/Payment/Payment_Author/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Payment/Payment_Author/Page_miFIN/img_Hi FARZANAN_userr'))

WebUI.click(findTestObject('Payment/Payment_Author/Page_miFIN/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

