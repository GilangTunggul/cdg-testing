<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add new customer</name>
   <tag></tag>
   <elementGuidId>5e1fc557-2653-451d-8f5e-d7b1b9ada79c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;name\&quot; : \&quot;name\&quot;,\n\t\&quot;email\&quot; : \&quot;asik@gmail.com\&quot;,\n\t\&quot;phone\&quot; : \&quot;628389349\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
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
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}api/cdg/specialist/customers</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>9011b1e4-1c69-4950-ad07-2aa17d33ad44</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokencsapp</defaultValue>
      <description></description>
      <id>513e4b32-6ee6-4087-b35a-4c21aae585b2</id>
      <masked>false</masked>
      <name>token</name>
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

def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())

if(jsonResponse.message == 'success') {

	WS.verifyResponseStatusCode(response, 200)
	
	assertThat(response.getStatusCode()).isEqualTo(200)
	
	WS.verifyElementPropertyValue(response, 'data', 'success')
	
	WS.verifyElementPropertyValue(response, 'message', 'success')

} else {

	WS.verifyResponseStatusCode(response, 400)
	
	assertThat(response.getStatusCode()).isEqualTo(400)
	
	WS.verifyElementPropertyValue(response, 'data', '[]')
	
	WS.verifyElementPropertyValue(response, 'message', 'Failed')

}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
