<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Submit device</name>
   <tag></tag>
   <elementGuidId>9b26e610-5d52-4a1c-90e6-b2c5ec1649fe</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>${cookies}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url}module/cdg/main/customer/ticket/submit/${ticketid}&amp;phone=${phone}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>4b3f01f1-adee-46d0-b203-274a69a70282</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>'5d958810d194552475437422'</defaultValue>
      <description></description>
      <id>9e9fd36c-aa5b-40ac-aaf1-26763d653b11</id>
      <masked>false</masked>
      <name>ticketid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cookies</defaultValue>
      <description></description>
      <id>32310a78-1365-4bb1-a2d1-b2374281782e</id>
      <masked>false</masked>
      <name>cookies</name>
   </variables>
   <variables>
      <defaultValue>'85707766742'</defaultValue>
      <description></description>
      <id>584fc06b-98fb-4742-80c0-841c0ab84a5e</id>
      <masked>false</masked>
      <name>phone</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())

def variables = request.getVariables()

def ticketid = variables.get('ticketid')

String data = jsonResponse.data

String value = '[]' 

if (data == value) {
	
	WS.verifyElementPropertyValue(response, 'message', '[Only new status can update to submitted status]')
		
	WS.verifyElementPropertyValue(response, 'data', '[]')
	
	WS.verifyElementPropertyValue(response, 'success', false)
	
} else {

WS.verifyElementPropertyValue(response, 'data', ticketid)

WS.verifyElementPropertyValue(response, 'success', true)

}




</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
