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

WebUI.setText(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/input_miFIN_userId'), 'COPSUSERM')

WebUI.setEncryptedText(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/button_LOGIN'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.maximizeWindow()

WebUI.click(findTestObject('Object Repository/Manual Advice/Maker/Page_DASHBOARD/i_DM VIEWER_img1000000017'))

WebUI.click(findTestObject('Object Repository/Manual Advice/Maker/Page_DASHBOARD/i_VIEWER_img1000001980'))

WebUI.click(findTestObject('Object Repository/Manual Advice/Maker/Page_DASHBOARD/a_MAKER'))

//WebUI.setText(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/input_DM Code_prospectNo'), 'DMFIN1000008259')
WebUI.setText(findTestObject('Manual Advice/Maker/Page_miFIN/input_National IDBRNPP No_generalFilter'), nic_num, FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/input_Revaluation Pending_search'))

WebUI.click(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/a_DMFIN1000008259'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/select_SelectReceivablePayable'), 
    'R', true)

WebUI.click(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/input_Send To Author_misBillId0'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/select_SelectADMINISTRATION FEEATTORNEY FEE_39dcd7'), 
    '1200003002', true)

WebUI.setText(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/input_Send To Author_chargeAmt0'), '100')

WebUI.setText(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/input_Send To Author_makeRemarks0'), 'ok')

WebUI.click(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/input_CHARGE DETAILS_blueBotton'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/select_SelectReceivablePayable_1'), 
    'P', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/select_SelectFRONT END FEE PAYABLEREGISTRAT_7fe3ce'), 
    '1200003056', true)

WebUI.setText(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/input_Send To Author_chargeAmt1'), '50')

WebUI.setText(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/input_Send To Author_makeRemarks1'), 'ok')

WebUI.click(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/input_Send To Author_select'))

WebUI.click(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.selectOptionByValue(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/select_SelectReceivablePayable'), 
    'R', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/select_SelectPRO RATA INTEREST'), 
    '1200003012', true)

WebUI.setText(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/input_Send To Author_makeRemarks0'), 'ok')

WebUI.click(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/input_Send To Author_select'))

WebUI.click(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.setText(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/input_Send To Author_chargeAmt0'), '50')

WebUI.click(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Manual Advice/Maker/Page_miFIN/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Object Repository/Manual Advice/Maker/Page_miFIN/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

WebUI.setText(findTestObject('Object Repository/Manual Advice/Author/Page_miFIN/input_miFIN_userId'), 'farzanan')

WebUI.setEncryptedText(findTestObject('Object Repository/Manual Advice/Author/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/Manual Advice/Author/Page_miFIN/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/Manual Advice/Author/Page_DASHBOARD/i_DM VIEWER_img1000000017'))

WebUI.click(findTestObject('Object Repository/Manual Advice/Author/Page_DASHBOARD/i_VIEWER_img1000001980'))

WebUI.click(findTestObject('Object Repository/Manual Advice/Author/Page_DASHBOARD/a_AUTHOR'))

//WebUI.setText(findTestObject('Object Repository/Manual Advice/Author/Page_miFIN/input_DM Code_prospectNo'), 'DMFIN1000008259')
WebUI.setText(findTestObject('Manual Advice/Author/Page_miFIN/input_National IDBRNPP No_generalFilter_Author'), nic_num, 
    FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Manual Advice/Author/Page_miFIN/input_Revaluation Pending_search'))

WebUI.click(findTestObject('Object Repository/Manual Advice/Author/Page_miFIN/a_DMFIN1000008259'))

WebUI.click(findTestObject('Object Repository/Manual Advice/Author/Page_miFIN/input_Selected_select'))

WebUI.scrollToElement(findTestObject('Manual Advice/Author/Page_miFIN/div_Approve'), 0)

WebUI.click(findTestObject('Object Repository/Manual Advice/Author/Page_miFIN/input_Approve_authAction'))

WebUI.setText(findTestObject('Object Repository/Manual Advice/Author/Page_miFIN/textarea_Remarks_arRemarks_1_2'), 'ok')

WebUI.click(findTestObject('Object Repository/Manual Advice/Author/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Manual Advice/Author/Page_miFIN/img_Hi FARZANAN_userr'))

WebUI.click(findTestObject('Object Repository/Manual Advice/Author/Page_miFIN/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

