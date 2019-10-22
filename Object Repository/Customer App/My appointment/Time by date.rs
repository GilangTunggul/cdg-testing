<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Time by date</name>
   <tag></tag>
   <elementGuidId>8298c150-7225-4e6c-9a3b-d69e0f992d6e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;date\&quot;: \&quot;1577379600000\&quot;,\n    \&quot;day\&quot;: \&quot;${day}\&quot;,\n    \&quot;storeID\&quot;: \&quot;${id}\&quot;\n}&quot;,
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
   <restUrl>${url}api/cdg/customer/appoinments/times</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>43af6be9-5dae-489e-a25e-5eddddc09897</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenapp</defaultValue>
      <description></description>
      <id>f65ae81a-39a4-49c4-b42f-9d9b0b131d34</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'5d14426fc7c95d266f00b892'</defaultValue>
      <description></description>
      <id>3a582eef-2c43-4c96-8f02-131322dc4633</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>'wednesday'</defaultValue>
      <description></description>
      <id>de26ae7b-80b5-4a1d-bb12-bb80c72b0167</id>
      <masked>false</masked>
      <name>day</name>
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

println(jsonResponse.data != null)

def variables = request.getVariables()

def id = variables.get('id')

def day = variables.get('day')

WS.verifyElementPropertyValue(response, 'data._id', id)

WS.verifyElementPropertyValue(response, 'data.day', day)

WS.verifyElementPropertyValue(response, 'data.timeBooked', jsonResponse.data.timeBooked)

WS.verifyElementPropertyValue(response, 'message', jsonResponse.message)

JsonSlurper slurper = new JsonSlurper()

Map parsedJson = slurper.parseText(response.getResponseText())

parsedJson.get('data.isHolidayOpen')

parsedJson.get('data.isActive')

parsedJson.get('message')



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
