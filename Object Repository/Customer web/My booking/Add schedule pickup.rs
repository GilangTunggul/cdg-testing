<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add schedule pickup</name>
   <tag></tag>
   <elementGuidId>5e269097-988e-4a6a-95af-85758fd85032</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;storeID&quot;,
      &quot;value&quot;: &quot;5d1441dcc7c95d27ed1eb182&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;scheduledAt&quot;,
      &quot;value&quot;: &quot;1566534075000&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;type&quot;,
      &quot;value&quot;: &quot;${type}&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;ticketID&quot;,
      &quot;value&quot;: &quot;${ticketid}&quot;,
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
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>${cookies}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}module/cdg/main/customer/appointments</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>5d88d687-ba27-4cc7-86d1-e9013a2433bd</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenapp</defaultValue>
      <description></description>
      <id>2a8ba084-f5b2-47aa-8ccf-878d15ff102c</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Appointment').getValue(1, 3)</defaultValue>
      <description></description>
      <id>6a88d0c1-9f11-43c8-8c38-6f8567682bc3</id>
      <masked>false</masked>
      <name>type</name>
   </variables>
   <variables>
      <defaultValue>'5ce4efc1c7c95d123a108033'</defaultValue>
      <description></description>
      <id>7dce7495-92ec-491f-84c8-ce815a987747</id>
      <masked>false</masked>
      <name>ticketid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cookies</defaultValue>
      <description></description>
      <id>52dc67ab-7e53-47e8-91da-cbdcf9ea9ede</id>
      <masked>false</masked>
      <name>cookies</name>
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

println(jsonResponse.data)

if(jsonResponse.data == []){
	
	WS.verifyElementPropertyValue(response, 'message', 'store not found')
	
} else {

	WS.verifyElementPropertyValue(response, 'message', '[]')
	
	WS.verifyElementPropertyValue(response, 'success', 'true')
}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
