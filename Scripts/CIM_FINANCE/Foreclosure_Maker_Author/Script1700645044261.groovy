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

WebUI.setText(findTestObject('Object Repository/Foreclosure/Maker/Page_miFIN/input_miFIN_userId'), 'copsuserm')

WebUI.setEncryptedText(findTestObject('Object Repository/Foreclosure/Maker/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/Foreclosure/Maker/Page_miFIN/button_LOGIN'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.maximizeWindow()

WebUI.click(findTestObject('Object Repository/Foreclosure/Maker/Page_DASHBOARD/i_DM VIEWER_img1000000017'))

WebUI.scrollToElement(findTestObject('Foreclosure/Maker/Page_DASHBOARD/a_LMS'), 0)

WebUI.click(findTestObject('Object Repository/Foreclosure/Maker/Page_DASHBOARD/i_VIEWER_img1000000059'))

WebUI.click(findTestObject('Object Repository/Foreclosure/Maker/Page_DASHBOARD/a_MAKER'))

/*/WebUI.setText(findTestObject('Object Repository/Foreclosure/Maker/Page_miFIN/input_National IDBRNPP No_generalFilter'), 
    'k3067154765078')/*/
WebUI.setText(findTestObject('Foreclosure/Maker/Page_miFIN/input_DM Code_prospectNo_1'), 'DMFIN1000007448')

WebUI.click(findTestObject('Object Repository/Foreclosure/Maker/Page_miFIN/input_Revaluation Pending_search'))

WebUI.click(findTestObject('Foreclosure/Maker/Page_miFIN/a_DMFIN1000008443'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Foreclosure/Maker/Page_miFIN/select_Select  SALE OF ASSETWRITE-OFFTOTAL _859ae1 (1)'), 
    '1000000004', true)

WebUI.click(findTestObject('Object Repository/Foreclosure/Maker/Page_miFIN/input_RV Sales Deed Release_rvSalesDeedRelease'))

WebUI.click(findTestObject('Object Repository/Foreclosure/Maker/Page_miFIN/input_FORECLOSURE DETAILS_compute_1'))

WebUI.scrollToElement(findTestObject('Foreclosure/Maker/Page_miFIN/div_ASSET DETAILS'), 0)

WebUI.delay(1)

/*/WebUI.selectOptionByValue(findTestObject('Object Repository/Foreclosure/Maker/Page_miFIN/select_SELECTLESSEETHIRD PARTYUSER'), 
    '1000000010', true)

WebUI.click(findTestObject('Object Repository/Foreclosure/Maker/Page_miFIN/input_Pincode_btnState'))

WebUI.switchToWindowTitle('Pincode')

WebUI.click(findTestObject('Object Repository/Foreclosure/Maker/Page_Pincode/font_Hindu House'))

WebUI.click(findTestObject('Object Repository/Foreclosure/Maker/Page_Pincode/input_Impasse Demerez_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.scrollToElement(findTestObject('Page_miFIN/div_State'), 0)/*/
WebUI.scrollToElement(findTestObject('Foreclosure/Maker/Page_miFIN/div_RECEIVABLE CHARGES'), 0)

//WebUI.scrollToElement(findTestObject('Page_miFIN/td_Foreclosure Interest'), 0)
WebUI.click(findTestObject('Object Repository/Foreclosure/Maker/Page_miFIN/input_Send To Author_mrSendToAuthor'))

WebUI.setText(findTestObject('Object Repository/Foreclosure/Maker/Page_miFIN/textarea_Remarks_mrRemarks'), 'ok')

WebUI.click(findTestObject('Object Repository/Foreclosure/Maker/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Foreclosure/Maker/Page_miFIN/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Object Repository/Foreclosure/Maker/Page_miFIN/a_Logout'))

WebUI.switchToWindowIndex(1)

WebUI.setText(findTestObject('Object Repository/Foreclosure/Author/Page_miFIN/input_miFIN_userId'), 'farzanan')

WebUI.setEncryptedText(findTestObject('Object Repository/Foreclosure/Author/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/Foreclosure/Author/Page_miFIN/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/Foreclosure/Author/Page_DASHBOARD/i_DM VIEWER_img1000000017'))

WebUI.scrollToElement(findTestObject('Foreclosure/Author/Page_DASHBOARD/a_LMS'), 0)

WebUI.click(findTestObject('Object Repository/Foreclosure/Author/Page_DASHBOARD/i_VIEWER_img1000000059'))

WebUI.click(findTestObject('Object Repository/Foreclosure/Author/Page_DASHBOARD/a_AUTHOR'))

WebUI.setText(findTestObject('Object Repository/Foreclosure/Author/Page_miFIN/input_DM Code_prospectNo'), 'DMFIN1000007448')

WebUI.click(findTestObject('Object Repository/Foreclosure/Author/Page_miFIN/input_Revaluation Pending_search'))

WebUI.click(findTestObject('Object Repository/Foreclosure/Author/Page_miFIN/a_DMFIN1000008443'))

WebUI.scrollToElement(findTestObject('Foreclosure/Author/Page_miFIN/td_Sr. No'), 0)

WebUI.scrollToElement(findTestObject('Foreclosure/Author/Page_miFIN/td_Outstanding Installment'), 0)

WebUI.click(findTestObject('Object Repository/Foreclosure/Author/Page_miFIN/input_Approve_arApprove'))

WebUI.setText(findTestObject('Object Repository/Foreclosure/Author/Page_miFIN/textarea_Remarks_arRemarks'), 'ok')

WebUI.click(findTestObject('Object Repository/Foreclosure/Author/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.acceptAlert()

WebUI.click(findTestObject('Foreclosure/Author/Page_miFIN/div_LEASE RESIDUAL VALUE UPLOAD_toggle-button'))

WebUI.click(findTestObject('Foreclosure/Author/Page_miFIN/i_VIEWER_img1200004000'))

WebUI.click(findTestObject('Foreclosure/Author/Page_miFIN/a_DM VIEWER'))

WebUI.setText(findTestObject('Foreclosure/Author/Page_miFIN/input_DM Code_dmCode'), 'DMFIN1000007448')

WebUI.click(findTestObject('Foreclosure/Author/Page_miFIN/input_Engine No_search_1'))

WebUI.delay(5)

WebUI.click(findTestObject('Foreclosure/Author/Page_miFIN/img_Hi FARZANAN_userr_1'))

WebUI.click(findTestObject('Foreclosure/Author/Page_miFIN/a_Logout_1'))

WebUI.closeBrowser()

