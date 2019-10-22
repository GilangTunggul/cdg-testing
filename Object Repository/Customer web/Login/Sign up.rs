<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Sign up</name>
   <tag></tag>
   <elementGuidId>18282bb6-bf95-4b92-8e51-61e1bc51454b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;email&quot;,
      &quot;value&quot;: &quot;${email}&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;userType&quot;,
      &quot;value&quot;: &quot;customer&quot;,
      &quot;type&quot;: &quot;Text&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}module/cdg/session/customer/signup/check</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>a46aabb0-133a-43c7-b2f5-99c37f86cd68</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cookies</defaultValue>
      <description></description>
      <id>8e500a0e-32c7-43cf-94c1-86bace8fa3c4</id>
      <masked>false</masked>
      <name>cookies</name>
   </variables>
   <variables>
      <defaultValue>'asikinaul@gmail.com'</defaultValue>
      <description></description>
      <id>0e511ee8-83a4-4ab1-adfd-bee08278a28c</id>
      <masked>false</masked>
      <name>email</name>
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

def variables = request.getVariables()

def email = variables.get('email')

def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())

String message =jsonResponse.success

String value = true

if ( message == value) {

	WS.verifyElementPropertyValue(response, 'message', '[Please check your email]')
	
	WS.verifyElementPropertyValue(response, 'data', jsonResponse.data)
	
	WS.verifyElementPropertyValue(response, 'data.redirect', 'signup')
	
	WS.verifyElementPropertyValue(response, 'success', 'true')
	
} else {

	WS.verifyElementPropertyValue(response, 'message', '[User is already exist]')
	
	WS.verifyElementPropertyValue(response, 'data', jsonResponse.data)
	
	WS.verifyElementPropertyValue(response, 'data.redirect', 'login')
	
	WS.verifyElementPropertyValue(response, 'success', 'false')

}

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
