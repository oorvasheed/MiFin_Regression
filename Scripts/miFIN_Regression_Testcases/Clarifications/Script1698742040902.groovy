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

WebUI.click(findTestObject('Object Repository/DM_Quotation_Creation/Page_miFIN/div_LEASE RESIDUAL VALUE UPLOAD_toggle-button (1)'))

WebUI.click(findTestObject('Object Repository/DM_Quotation_Creation/Page_miFIN/i_DM APPLICATION_img1200004003'))

WebUI.click(findTestObject('Object Repository/DM_Quotation_Creation/Page_miFIN/a_CASHFLOW (1)'))

WebUI.selectOptionByValue(findTestObject('Object Repository/DM_Quotation_Creation/Page_miFIN/select_SELECT PENDINGDISBURSEDCLOSEDFORECLO_ad6911 (1)'), 
    '1000000001', true)

WebUI.click(findTestObject('Object Repository/DM_Quotation_Creation/Page_miFIN/input_Engine No_search (3)'))

WebUI.click(findTestObject('Object Repository/DM_Quotation_Creation/Page_miFIN/a_DMFIN1000008405'))

WebUI.click(findTestObject('Object Repository/DM_Quotation_Creation/Page_miFIN/a_QUOTATION'))

WebUI.doubleClick(findTestObject('Object Repository/DM_Quotation_Creation/Page_miFIN/input_Effective Lease Start Date_START_DATE'))

WebUI.setText(findTestObject('Object Repository/DM_Quotation_Creation/Page_miFIN/input_Effective Lease Start Date_START_DATE'), 
    '30-jul-2023')

WebUI.selectOptionByValue(findTestObject('Object Repository/DM_Quotation_Creation/Page_miFIN/select_SELECTMONTHLY'), '1000000004', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/DM_Quotation_Creation/Page_miFIN/select_SELECT17142128'), '7', 
    true)

WebUI.click(findTestObject('Object Repository/DM_Quotation_Creation/Page_miFIN/a_Save (1)'))

