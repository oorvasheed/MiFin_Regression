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

WebUI.setText(findTestObject('Object Repository/CR Approval/input_miFIN_userId'), 'farzanan')

WebUI.setEncryptedText(findTestObject('Object Repository/CR Approval/input_miFIN_password'), 'iZKiu3Mw15dMyU9HHbuK3g==')

WebUI.click(findTestObject('Object Repository/CR Approval/button_LOGIN'))

WebUI.acceptAlert(FailureHandling.OPTIONAL)

WebUI.switchToWindowTitle('DASHBOARD')

WebUI.click(findTestObject('Object Repository/CR Approval/i_CHANGE PASSWORD_img1000002041'))

WebUI.maximizeWindow()

WebUI.click(findTestObject('Object Repository/CR Approval/i_CUSTOMER_img1000010288'))

WebUI.click(findTestObject('Object Repository/CR Approval/a_CR REVIEW'))

WebUI.setText(findTestObject('Object Repository/CR Approval/input_Last Name_lastName'), last_name)

WebUI.click(findTestObject('Object Repository/CR Approval/input_National IDBRNPP No_search'))

WebUI.click(findTestObject('Object Repository/CR Approval/a_CNCIMF000002263'))

WebUI.selectOptionByValue(findTestObject('Object Repository/CR Approval/select_SELECTREJECT APPROVE SEND BACK TO PR_0bbc91'), 
    '1000000004', true)

WebUI.setText(findTestObject('Object Repository/CR Approval/textarea_Remarks_remark_1'), 'ok')

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/additional items/a_Save'))

WebUI.waitForAlert(2)

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/CR Approval/img_Hi FARZANAN_userr'))

WebUI.click(findTestObject('Page_miFIN/a_Logout (1)'))

WebUI.closeBrowser()

