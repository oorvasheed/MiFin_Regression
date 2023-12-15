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

WebUI.setText(findTestObject('Object Repository/New Group/input_miFIN_userId'), 'Copsuserm')

WebUI.setEncryptedText(findTestObject('Object Repository/New Group/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/New Group/button_LOGIN'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/New Group/i_CUSTOMER VIEWER_img1100106515'))

WebUI.click(findTestObject('Object Repository/New Group/a_NEW GROUP'))

'Set group name here'
WebUI.setText(findTestObject('New Group/input_Group Name_createGroupId'), group_name)

WebUI.selectOptionByValue(findTestObject('Object Repository/New Group/select_SELECTCORPORATE'), '1000000002', true)

WebUI.click(findTestObject('New Group/input_Group Type_Create Group'))

WebUI.waitForAlert(3, FailureHandling.OPTIONAL)

WebUI.acceptAlert()

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('New Group/input_Group Type_groupCode'))

WebUI.click(findTestObject('Object Repository/New Group/a_Continue'))

WebUI.setText(findTestObject('Object Repository/New Group/input_Company Name First Name_firstName_1_2_3_4'), first_name)

WebUI.setText(findTestObject('Object Repository/New Group/input_Last Name_lastName'), last_name)

WebUI.click(findTestObject('Object Repository/New Group/input_Date of BirthIncorporation_blueBotton'))

WebUI.click(findTestObject('Object Repository/New Group/input_MOBILE NO_selectApplicant'))

WebUI.click(findTestObject('Object Repository/New Group/input_TESTING  TEST_blueBotton'))

WebUI.waitForAlert(1, FailureHandling.OPTIONAL)

WebUI.acceptAlert()

WebUI.takeScreenshot(FailureHandling.OPTIONAL)

WebUI.click(findTestObject('New Group/Page_miFIN/img_Hi COPSUSERM_userr'))

WebUI.click(findTestObject('New Group/Page_miFIN/a_Logout'))

WebUI.switchToWindowTitle('miFIN')

