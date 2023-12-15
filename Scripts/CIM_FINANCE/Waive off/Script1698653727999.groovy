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

WebUI.setText(findTestObject('Object Repository/Waive-off/Maker/Page_miFIN/input_miFIN_userId'), 'Copsuserm')

WebUI.setEncryptedText(findTestObject('Object Repository/Waive-off/Maker/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/Waive-off/Maker/Page_miFIN/button_LOGIN'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Waive-off/Maker/Page_DASHBOARD/i_VIEWER_img1200004000'))

WebUI.click(findTestObject('Waive-off/Maker/Page_DASHBOARD/a_DM VIEWER'))

WebUI.setText(findTestObject('Waive-off/Maker/Page_miFIN/input_National IDBRNPP No_generalFilter'), nic_num)

WebUI.click(findTestObject('Waive-off/Maker/Page_miFIN/input_Engine No_search_1(1)'))

String dmCode = WebUI.getText(findTestObject('Waive-off/Maker/Page_miFIN/a_DMFIN1000008597'))

WebUI.click(findTestObject('Waive-off/Maker/Page_DASHBOARD/i_DM VIEWER_img1000000017'))

WebUI.scrollToElement(findTestObject('Waive-off/Maker/Page_DASHBOARD/a_LMS'), 0)

WebUI.click(findTestObject('Object Repository/Waive-off/Maker/Page_DASHBOARD/i_VIEWER_img1000000049'))

WebUI.click(findTestObject('Object Repository/Waive-off/Maker/Page_DASHBOARD/a_MAKER'))

WebUI.setText(findTestObject('Waive-off/Maker/Page_miFIN/input_DM Code_prospectNo_(1)'), dmCode)

//WebUI.setText(findTestObject('Waive-off/Maker/Page_miFIN/input_National IDBRNPP No_generalFilter_1'), nic_num)
WebUI.click(findTestObject('Object Repository/Waive-off/Maker/Page_miFIN/input_Revaluation Pending_search'))

WebUI.click(findTestObject('Object Repository/Waive-off/Maker/Page_miFIN/a_DMFIN1000008482'))

WebUI.click(findTestObject('Waive-off/Maker/Page_miFIN/input_Remarks_waiveOffCumulative_1'))

WebUI.selectOptionByValue(findTestObject('Waive-off/Maker/Page_miFIN/select_SELECT     GOOD  RECORDCUSTOMER REQUESTPROMOTION SCHEME_1'), 
    '1000000003', true)

WebUI.setText(findTestObject('Waive-off/Maker/Page_miFIN/input_Remarks_txtf_waiveOffRemarksCumulative_1'), 'ok')

/*/WebUI.click(findTestObject('Waive-off/Maker/Page_miFIN/input_Remarks_waiveOffCumulative(6)'), FailureHandling.OPTIONAL)

WebUI.selectOptionByValue(findTestObject('Waive-off/Maker/Page_miFIN/select_SELECT GOOD RECORDCUSTOMER REQU_b2dec2(6)'), 
    '1000000001', true)

WebUI.setText(findTestObject('Waive-off/Maker/Page_miFIN/input_Remarks_txtf_waiveOffRemarksCumulative(6)'), 'ok')/*/
WebUI.scrollToElement(findTestObject('Waive-off/Maker/Page_miFIN/div_MAKERS REMARKS'), 0)

WebUI.click(findTestObject('Object Repository/Waive-off/Maker/Page_miFIN/input_Send To Author_mrSendToAuthor'))

WebUI.setText(findTestObject('Object Repository/Waive-off/Maker/Page_miFIN/textarea_Remarks_mrRemarks'), 'ok')

WebUI.click(findTestObject('Object Repository/Waive-off/Maker/Page_miFIN/a_Save  Exit'))

WebUI.acceptAlert()

WebUI.click(findTestObject('Waive-off/Maker/Page_miFIN/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Waive-off/Maker/Page_miFIN/a_Logout_1'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/Waive-off/Author/Page_miFIN/input_miFIN_userId'), 'farzanan')

WebUI.setEncryptedText(findTestObject('Object Repository/Waive-off/Author/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/Waive-off/Author/Page_miFIN/button_LOGIN'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/Waive-off/Author/Page_DASHBOARD/i_DM VIEWER_img1000000017'))

WebUI.click(findTestObject('Object Repository/Waive-off/Author/Page_DASHBOARD/i_VIEWER_img1000000049'))

WebUI.click(findTestObject('Object Repository/Waive-off/Author/Page_DASHBOARD/a_AUTHOR'))

WebUI.setText(findTestObject('Waive-off/Author/Page_miFIN/input_National IDBRNPP No_generalFilter_1'), nic_num)

WebUI.click(findTestObject('Object Repository/Waive-off/Author/Page_miFIN/input_Revaluation Pending_search'))

WebUI.click(findTestObject('Object Repository/Waive-off/Author/Page_miFIN/a_DMFIN1000008299'))

WebUI.scrollToElement(findTestObject('Waive-off/Author/Page_miFIN/div_AUTHORS REMARKS'), 0)

WebUI.click(findTestObject('Object Repository/Waive-off/Author/Page_miFIN/input_Approve_arApprove'))

WebUI.setText(findTestObject('Object Repository/Waive-off/Author/Page_miFIN/textarea_Remarks_arRemarks_1'), 'ok')

WebUI.click(findTestObject('Object Repository/Waive-off/Author/Page_miFIN/a_Save  Exit'))

WebUI.acceptAlert()

WebUI.delay(3, FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Waive-off/Author/Page_miFIN/img_Hi FARZANAN_userr'))

WebUI.click(findTestObject('Object Repository/Waive-off/Author/Page_miFIN/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

