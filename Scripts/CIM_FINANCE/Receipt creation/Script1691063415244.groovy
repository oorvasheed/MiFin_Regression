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

WebUI.setText(findTestObject('Object Repository/all in one/input_miFIN_userId (1)'), 'copsuserm')

WebUI.setEncryptedText(findTestObject('Object Repository/all in one/input_miFIN_password (1)'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/all in one/button_LOGIN (1)'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.maximizeWindow()

WebUI.click(findTestObject('Object Repository/all in one/i_DM VIEWER_img1000000017 (1)'))

WebUI.click(findTestObject('Object Repository/all in one/i_MAKER_img1000000034 (1)'))

WebUI.click(findTestObject('Object Repository/Page_DASHBOARD/a_MAKER'))

WebUI.setText(findTestObject('Object Repository/Page_miFIN/input_National IDBRNPP No_generalFilter (1)'), nic_num)

WebUI.click(findTestObject('Object Repository/all in one/input_Revaluation Pending_search (1)'))

WebUI.click(findTestObject('Object Repository/all in one/a_DMFIN1000008259_1 (1)'))

WebUI.setText(findTestObject('Object Repository/all in one/input__txtf_rcReceiptAmount_1_2_3 (1)'), '500')

WebUI.click(findTestObject('Object Repository/Page_miFIN/input__txtf_rcReceiptDate'))

WebUI.delay(2)

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_miFIN/select_SELECT     CUSTOMERTHIRD PARTY'), 5, FailureHandling.OPTIONAL)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECT     CUSTOMERTHIRD PARTY'), '1000000001', 
    false)

WebUI.click(findTestObject('Object Repository/all in one/div_Entity Type  SELECT     CUSTOMERTHIRD P_137bcf'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_miFIN/select_SELECT     CASHBANK TRANSFERCHEQUECR_23534d'), 
    '1000000002', true)

WebUI.scrollToElement(findTestObject('Object Repository/Page_miFIN/input__btnDepositeBankName'), 0)

WebUI.click(findTestObject('Object Repository/Page_miFIN/input__btnDepositeBankName'))

WebUI.switchToWindowTitle('Bank')

WebUI.click(findTestObject('Object Repository/all in one/Page_miFIN/Page_Bank/font_CIM BANK'))

WebUI.click(findTestObject('Object Repository/all in one/Page_miFIN/Page_Bank/input_STANDARD BANK (MAURITIUS) LTD_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/Page_miFIN/input__btnDepositeBranchName'))

WebUI.switchToWindowTitle('Branch')

WebUI.click(findTestObject('Object Repository/Page_Branch/font_PORT LOUIS'))

WebUI.click(findTestObject('Object Repository/Page_Branch/input_PORT LOUIS_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/Page_miFIN/input__btnAutoAccountName'))

WebUI.switchToWindowTitle('Account')

WebUI.click(findTestObject('Object Repository/all in one/font_123456789 (1)'))

WebUI.click(findTestObject('Object Repository/Page_Account/input_Name_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Object Repository/Page_miFIN/input_Send To Author_mrSendToAuthor'))

WebUI.setText(findTestObject('Object Repository/Page_miFIN/textarea_Remarks_mrRemarks'), 'ok')

WebUI.switchToWindowTitle('miFIN')

WebUI.scrollToElement(findTestObject('Object Repository/Page_miFIN/input__txtf_rcReceiptDate'), 0)

WebUI.click(findTestObject('Object Repository/Page_miFIN/input__btnBranch'))

WebUI.switchToWindowTitle('Branch/Cashpoint')

WebUI.click(findTestObject('Object Repository/Page_BranchCashpoint/font_CASH AND DISCOUNT'))

WebUI.click(findTestObject('Object Repository/Page_BranchCashpoint/input_Y CADER CO LTD PORT LOUIS_OK'))

WebUI.switchToWindowTitle('miFIN')

WebUI.click(findTestObject('Receipt maker/Page_miFIN/a_Save'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

WebUI.click(findTestObject('Receipt maker/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Receipt maker/a_Logout (3)'))

WebUI.switchToWindowTitle('miFIN')

