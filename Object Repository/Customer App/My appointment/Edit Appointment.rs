<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Edit Appointment</name>
   <tag></tag>
   <elementGuidId>a31eca8b-043e-4dd0-a1ee-29b111f847db</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;action\&quot;: \&quot;update\&quot;,\n    \&quot;set\&quot;: {\n        \&quot;id\&quot;: \&quot;${id}\&quot;,\n        \&quot;description\&quot;: \&quot;test descptions\&quot;,\n        \&quot;storeID\&quot;: \&quot;5c57afd44d8e2030156aece6\&quot;,\n        \&quot;scheduledAt\&quot;: \&quot;1559266200000\&quot;\n    },\n    \&quot;ticket\&quot; : {\n        \&quot;_id\&quot; : \&quot;5ce4e5bd3f6dee005573ffe2\&quot;,\n        \&quot;scheduleSubmitted\&quot; : \&quot;1559266200000\&quot;\n    }\n}&quot;,
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
      <value>${cookies}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${url}api/cdg/customer/appoinments</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>e7105c2c-4543-49b6-aa09-eeb282b7d182</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenapp</defaultValue>
      <description></description>
      <id>c8be9855-c83c-418e-a263-dcbcdb63945b</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'5ce4e5bd3f6dee005573ffe2'</defaultValue>
      <description></description>
      <id>ce9b5598-c538-486b-98c4-f0abd94c511c</id>
      <masked>false</masked>
      <name>id</name>
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

def id = variables.get('id')

WS.verifyElementPropertyValue(response, 'data', id)

WS.verifyElementPropertyValue(response, 'message', 'success')

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
