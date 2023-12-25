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
import groovy.json.JsonSlurper as JsonSlurper
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import java.nio.file.WatchService as WatchService
import java.sql.Connection as Connection
import java.sql.ResultSet as ResultSet
import com.kms.katalon.core.configuration.RunConfiguration as RunConfiguration
import com.katalon.plugin.keyword.connection.DBType as DBType

// Send the first request
def authResponse = sendRequestAndHandleResponse('AR BE/Auth', [('shop') : raw_domain])

def authorization = extractAuthorizationToken(authResponse)

print(authorization)

// Send the second request
def shopInfoResponse = sendRequestAndHandleResponse(object_repo, [('authorization') : authorization, ('payload') : request_payload])

//check response status code
WS.verifyResponseStatusCode(shopInfoResponse, response_status)

// Parse and log the shop name

//KeywordUtil.logInfo(shopInfo.data.shopInfo.shop_name)
//WS.validateJsonAgainstSchema(null, null, FailureHandling.STOP_ON_FAILURE)

response = WS.sendRequest(findTestObject('AR Reviews Widget Hub/shopify metafields', [('raw_domain') : raw_domain
            , ('access_token') : access_token]))

def metafields =  textToJson(response.getResponseBodyContent())
//println metafields.data.currentAppInstallation.review_box_setting.value
def metafields_value =  textToJson(metafields.data.currentAppInstallation.review_box_setting.value)

metafields_value.each { key, value ->
	println "Property: $key, Value: $value"
}

request_payload_obj =  textToJson(request_payload)
def diffProperties = []
request_payload_obj.setting.each { key, value1 ->
	println key
	def value2 = metafields_value[key]
	println "value:"+ value2
	
	if ( value1 != value2) {
		diffProperties << key
	}
}
println diffProperties
KeywordUtil.logInfo(diffProperties.toString())

def extractAuthorizationToken(def response) {
    def jsonResponse =  new JsonSlurper().parseText(response.getResponseBodyContent())

    return 'Bearer ' + jsonResponse.data.token
}

def sendRequestAndHandleResponse(def testObject, def additionalParams = [:]) {
    def response = WS.sendRequest(findTestObject(testObject, additionalParams))

    return response
}

def textToJson(def text_object) {
	return new JsonSlurper().parseText(text_object)
}

