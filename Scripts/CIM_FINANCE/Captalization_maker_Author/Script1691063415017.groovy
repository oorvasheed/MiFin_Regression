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

WebUI.setText(findTestObject('Object Repository/Post_sanction_Doc/Page_miFIN/input_miFIN_userId'), 'COPSUSERM')

WebUI.setEncryptedText(findTestObject('Object Repository/Post_sanction_Doc/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/Post_sanction_Doc/Page_miFIN/button_LOGIN'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/Post_sanction_Doc/Page_DASHBOARD/i_VIEWER_img1200004000'))

WebUI.click(findTestObject('Object Repository/Post_sanction_Doc/Page_DASHBOARD/i_DM APPLICATION_img1200004003'))

WebUI.click(findTestObject('Object Repository/Post_sanction_Doc/Page_miFIN/a_CAPITALISATION  MAKER'))

WebUI.setText(findTestObject('Object Repository/Capatialization/Page_miFIN/input_Customer Name_firstName_1'), last_name)

WebUI.click(findTestObject('Object Repository/Capatialization/Page_miFIN/input_Delivery Date_search'))

WebUI.click(findTestObject('Object Repository/Capatialization/Page_miFIN/a_DMFIN1000008506'))

WebUI.click(findTestObject('Object Repository/Capatialization/Page_miFIN/input_Capitalised_temp_capitalized'))

WebUI.click(findTestObject('Object Repository/Capatialization/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Capatialization/Page_miFIN/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('Capatialization/Page_miFIN/a_Logout_3'))

WebUI.switchToWindowIndex(0)

WebUI.setText(findTestObject('Object Repository/Capatialization/Page_miFIN/input_miFIN_userId'), 'farzanan')

WebUI.setEncryptedText(findTestObject('Object Repository/Capatialization/Page_miFIN/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/Capatialization/Page_miFIN/button_LOGIN'))

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/Capatialization/Page_DASHBOARD/i_VIEWER_img1200004000'))

WebUI.click(findTestObject('Object Repository/Capatialization/Page_DASHBOARD/i_DM APPLICATION_img1200004003'))

WebUI.click(findTestObject('Object Repository/Capatialization/Page_DASHBOARD/a_CAPITALISATION  AUTHOR'))

WebUI.setText(findTestObject('Object Repository/Capatialization/Page_miFIN/input_Customer Name_firstName_1'), last_name)

WebUI.click(findTestObject('Object Repository/Capatialization/Page_miFIN/input_Delivery Date_search'))

WebUI.click(findTestObject('Object Repository/Capatialization/Page_miFIN/a_DMFIN1000008506'))

WebUI.click(findTestObject('Object Repository/Capatialization/Page_miFIN/a_Save'))

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Capatialization/Page_miFIN/img_Hi FARZANAN_userr'))

WebUI.click(findTestObject('Object Repository/Capatialization/Page_miFIN/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

